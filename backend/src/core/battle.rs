use crate::core::character::Character;
use crate::core::spells::{self, Spell, SpellOutcome, SpellTarget};
use crate::core::status::StatusEffects;
use crate::core::turn_queue::{CombatantId, TeamId, TurnQueue};
use anyhow::Result;
use rand::Rng;
use serde::Serialize;

const SPELL_CHANCE: f64 = 1.0 / 3.0;

const COMBATANT_A: CombatantId = CombatantId(0);
const COMBATANT_B: CombatantId = CombatantId(1);

#[derive(Serialize, Clone, Debug)]
pub struct CombatantSnapshot {
    pub hp: i32,
    pub defense: i32,
    pub status_list: StatusEffects,
}

#[derive(Serialize, Clone, Debug)]
pub struct TurnLog {
    pub turn_number: u32,
    pub attacker_team: TeamId,
    pub base_damage: i32,
    pub defender_defense_bonus_consumed: i32,
    pub spell_triggered: Option<Spell>,
    pub spell_description: Option<String>,
    pub spell_defense_bonus_promised: i32,
    pub character_a_after: CombatantSnapshot,
    pub character_b_after: CombatantSnapshot,
}

#[derive(Serialize, Debug)]
pub struct BattleResult {
    pub character_a_start: Character,
    pub character_b_start: Character,
    pub turns: Vec<TurnLog>,
    pub winner_team: TeamId,
}

pub fn simulate_battle(
    char_a: Character,
    char_b: Character,
    rng: &mut impl Rng,
) -> Result<BattleResult> {
    let mut a = char_a.clone();
    let mut b = char_b.clone();

    let mut queue = TurnQueue::new_1vs1(COMBATANT_A, COMBATANT_B, rng);
    let mut turns = Vec::new();
    let mut turn_number = 1;

    loop {
        let attacker_id = queue.current_attacker();
        let attacker_team = queue.team_of(attacker_id)?;

        let (attacker, defender) = match attacker_team {
            TeamId::A => (&mut a, &mut b),
            TeamId::B => (&mut b, &mut a),
        };

        let defense_bonus = defender.status_list.consume_defense_bonus();
        let effective_defense = defender.defense + defense_bonus;

        let base_damage = (attacker.strength - effective_defense).max(0);
        defender.take_damage(base_damage);

        let mut spell_triggered = None;
        let mut spell_description = None;
        let mut instant_win = false;
        let mut grants_extra_turn = false;
        let mut spell_defense_bonus_promised = 0;

        if defender.is_alive() && rng.gen_bool(SPELL_CHANCE) {
            let spell = attacker.spell;
            let outcome: SpellOutcome = spells::cast(spell, attacker, defender, rng);

            apply_spell_outcome(&outcome, attacker, defender);

            spell_triggered = Some(spell);
            spell_description = Some(outcome.description);
            instant_win = outcome.instant_win;
            grants_extra_turn = outcome.grants_extra_turn;
            spell_defense_bonus_promised = outcome.defense_bonus_promised;
        }

        let (character_a_after, character_b_after) = match attacker_team {
            TeamId::A => (
                CombatantSnapshot {
                    hp: attacker.hp,
                    defense: attacker.defense,
                    status_list: attacker.status_list.clone(),
                },
                CombatantSnapshot {
                    hp: defender.hp,
                    defense: defender.defense,
                    status_list: defender.status_list.clone(),
                },
            ),
            TeamId::B => (
                CombatantSnapshot {
                    hp: defender.hp,
                    defense: defender.defense,
                    status_list: defender.status_list.clone(),
                },
                CombatantSnapshot {
                    hp: attacker.hp,
                    defense: attacker.defense,
                    status_list: attacker.status_list.clone(),
                },
            ),
        };

        turns.push(TurnLog {
            turn_number,
            attacker_team,
            base_damage,
            defender_defense_bonus_consumed: defense_bonus,
            spell_triggered,
            spell_description,
            spell_defense_bonus_promised: spell_defense_bonus_promised,
            character_a_after,
            character_b_after,
        });

        if instant_win || !defender.is_alive() {
            let winner_team = attacker_team;
            return Ok(BattleResult {
                character_a_start: char_a,
                character_b_start: char_b,
                turns,
                winner_team,
            });
        }

        if grants_extra_turn {
            queue.grant_extra_turn(attacker_id)?;
        } else {
            queue.advance_after_turn(attacker_id)?;
        }

        turn_number += 1;
    }
}

fn apply_spell_outcome(outcome: &SpellOutcome, caster: &mut Character, target: &mut Character) {
    if outcome.extra_damage > 0 {
        target.take_damage(outcome.extra_damage);
    }
    if outcome.full_heal {
        caster.heal_to_full();
    }
    if outcome.heal_amount > 0 {
        caster.heal(outcome.heal_amount);
    }
    if outcome.defense_increase > 0 {
        caster.increase_defense(outcome.defense_increase);
    }
    if let Some((who, status)) = outcome.status_to_apply {
        match who {
            SpellTarget::Caster => caster.status_list.add(status),
            SpellTarget::Target => target.status_list.add(status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::status::{StatusEffect, StatusKind};
    use rand::rngs::mock::StepRng;
    use std::collections::VecDeque;

    // StepRng always returning 0 forces every gen_bool call to true
    // any probability > 0 passes
    fn always_true_rng() -> StepRng {
        StepRng::new(0, 0)
    }

    // Fixed word acts as a list of gen_bool outcomes
    // first N calls consume words in order
    // every later call falls back to fallback
    //
    // This lets us do "who attacks first"
    // independently from "does the spell trigger this turn"
    struct ScriptedRng {
        words: VecDeque<u64>,
        fallback: u64,
    }

    impl ScriptedRng {
        fn new(words: Vec<u64>, fallback: u64) -> Self {
            ScriptedRng {
                words: words.into(),
                fallback,
            }
        }
    }

    impl rand::RngCore for ScriptedRng {
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.words.pop_front().unwrap_or(self.fallback)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for chunk in dest.chunks_mut(8) {
                let bytes = self.next_u64().to_le_bytes();
                chunk.copy_from_slice(&bytes[..chunk.len()]);
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    // Word 0 forces gen_bool to true
    // Coin flip picks character A to go first
    // Then it falls back to false so:
    // - no spell triggers
    // - damage stays predictable
    fn a_first_no_spells_rng() -> ScriptedRng {
        ScriptedRng::new(vec![0], u64::MAX)
    }

    fn character(name: &str, hp: i32, strength: i32, defense: i32, spell: Spell) -> Character {
        Character {
            name: name.to_string(),
            hp,
            max_hp: hp,
            strength,
            defense,
            spell,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn attacker_deals_strength_minus_defense_damage() {
        // No spells trigger and A is guaranteed to attack first,
        // so we can predict damage deterministically
        let a = character("A", 100, 20, 5, Spell::HardHit);
        let b = character("B", 3, 20, 5, Spell::HardHit);
        let mut rng = a_first_no_spells_rng();

        let result = simulate_battle(a, b, &mut rng).unwrap();

        // First turn:
        // A attacks B (base_damage = 20 - 5 = 15), which
        // finishes B (hp = 3)
        assert_eq!(result.turns[0].attacker_team, TeamId::A);
        assert_eq!(result.turns[0].base_damage, 15);
        assert_eq!(result.turns.len(), 1);
        assert_eq!(result.winner_team, TeamId::A);
    }

    #[test]
    fn damage_is_floored_at_zero_when_defense_exceeds_strength() {
        let a = character("A", 40, 1, 5, Spell::HardHit);
        let b = character("B", 100, 30, 20, Spell::HardHit);
        let mut rng = a_first_no_spells_rng();

        let result = simulate_battle(a, b, &mut rng).unwrap();

        for turn in &result.turns {
            match turn.attacker_team {
                TeamId::A => assert_eq!(
                    turn.base_damage, 0,
                    "A's attack should always be floored to 0"
                ),
                TeamId::B => assert_eq!(turn.base_damage, 25, "B always deals 30 - 5 = 25"),
            }
        }
        assert_eq!(result.winner_team, TeamId::B);
    }

    #[test]
    fn battle_ends_when_defender_hp_reaches_zero_and_reports_correct_winner() {
        let a = character("A", 100, 50, 0, Spell::HardHit);
        let b = character("B", 10, 5, 0, Spell::HardHit);
        let mut rng = a_first_no_spells_rng();

        let result = simulate_battle(a, b, &mut rng).unwrap();

        assert_eq!(result.winner_team, TeamId::A);
        assert!(!result.turns.is_empty());

        let last_turn = result.turns.last().unwrap();
        assert_eq!(last_turn.character_b_after.hp, 0);
    }

    #[test]
    fn pre_existing_defense_bonus_is_consumed_and_reduces_damage() {
        let a = character("A", 100, 20, 0, Spell::HardHit);
        let mut b = character("B", 100, 20, 0, Spell::HardHit);

        b.status_list
            .add(StatusEffect::once(StatusKind::ExtraDefenseAura {
                amount: 5,
            }));

        let mut rng = a_first_no_spells_rng();

        let result = simulate_battle(a, b, &mut rng).unwrap();

        // A attacks first
        // B (defender) consumes its aura
        let first_turn = &result.turns[0];
        assert_eq!(first_turn.attacker_team, TeamId::A);
        assert_eq!(first_turn.defender_defense_bonus_consumed, 5);
        assert_eq!(first_turn.base_damage, 15); // 20 strength - (0 + 5) defense
        // Aura was single use, should be gone
        assert!(first_turn.character_b_after.status_list.is_empty());
    }

    #[test]
    fn critical_hit_spell_ends_battle_instantly_even_at_full_hp() {
        let a = character("A", 100, 20, 0, Spell::CriticalHit);
        let b = character("B", 100, 20, 0, Spell::HardHit);

        let mut rng = always_true_rng();

        let result = simulate_battle(a, b, &mut rng).unwrap();

        assert_eq!(result.winner_team, TeamId::A);
        assert_eq!(result.turns.len(), 1);
        assert!(result.turns[0].character_b_after.hp > 0);
    }

    #[test]
    fn rage_spell_grants_the_same_attacker_another_turn() {
        let a = character("A", 100, 10, 0, Spell::Rage);
        let b = character("B", 100, 10, 0, Spell::HardHit);
        let mut rng = always_true_rng();

        let result = simulate_battle(a, b, &mut rng).unwrap();

        assert_eq!(result.turns[0].attacker_team, TeamId::A);
        assert_eq!(result.turns[1].attacker_team, TeamId::A);
    }

    #[test]
    fn apply_spell_outcome_applies_extra_damage_heal_and_defense() {
        let mut caster = character("Caster", 50, 20, 5, Spell::HardHit);
        let mut target = character("Target", 50, 20, 5, Spell::HardHit);

        let outcome = SpellOutcome {
            extra_damage: 10,
            heal_amount: 4,
            defense_increase: 2,
            ..SpellOutcome::just("test")
        };

        apply_spell_outcome(&outcome, &mut caster, &mut target);

        assert_eq!(target.hp, 40);
        assert_eq!(caster.hp, 50); // 50 == max, capped
        assert_eq!(caster.defense, 7);
    }

    #[test]
    fn apply_spell_outcome_full_heal_restores_caster_to_max_hp() {
        let mut caster = character("Caster", 50, 20, 5, Spell::HardHit);
        let mut target = character("Target", 50, 20, 5, Spell::HardHit);
        caster.max_hp = 50;

        let outcome = SpellOutcome {
            full_heal: true,
            ..SpellOutcome::just("test")
        };

        apply_spell_outcome(&outcome, &mut caster, &mut target);

        assert_eq!(caster.hp, 50);
    }

    #[test]
    fn apply_spell_outcome_applies_status_to_the_correct_side() {
        let mut caster = character("Caster", 50, 20, 5, Spell::HardHit);
        let mut target = character("Target", 50, 20, 5, Spell::HardHit);

        let status = StatusEffect::once(StatusKind::ExtraDefenseAura { amount: 5 });
        let outcome = SpellOutcome {
            status_to_apply: Some((SpellTarget::Caster, status)),
            ..SpellOutcome::just("test")
        };

        apply_spell_outcome(&outcome, &mut caster, &mut target);

        assert!(!caster.status_list.is_empty());
        assert!(target.status_list.is_empty());
    }
}
