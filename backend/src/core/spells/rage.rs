use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct Rage;

impl SpellEffect for Rage {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        if rng.gen_bool(1.0 / 5.0) {
            SpellOutcome {
                grants_extra_turn: true,
                ..SpellOutcome::just("Attacker is enraged!")
            }
        } else {
            SpellOutcome::just("Attacker is not angry enough!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::character::Character;
    use crate::core::spells::Spell;
    use crate::core::status::StatusEffects;
    use rand::rngs::mock::StepRng;

    fn dummy_character() -> Character {
        Character {
            name: "Dummy".to_string(),
            hp: 50,
            max_hp: 50,
            strength: 20,
            defense: 2,
            spell: Spell::Rage,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn success_grants_extra_turn() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(0, 0);

        let outcome = Rage.cast(&caster, &target, &mut rng);

        assert!(outcome.grants_extra_turn);
    }

    #[test]
    fn failure_does_not_grant_extra_turn() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(u64::MAX, 0);

        let outcome = Rage.cast(&caster, &target, &mut rng);

        assert!(!outcome.grants_extra_turn);
    }
}
