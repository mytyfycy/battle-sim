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
