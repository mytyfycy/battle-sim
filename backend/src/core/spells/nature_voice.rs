use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct NatureVoice;

impl SpellEffect for NatureVoice {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        if rng.gen_bool(1.0 / 10.0) {
            SpellOutcome {
                full_heal: true,
                ..SpellOutcome::just("Attacker's health is fully restored!")
            }
        } else {
            SpellOutcome::just("Attacker failed to restore its health!")
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
            spell: Spell::NatureVoice,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn success_grants_full_heal() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(0, 0);

        let outcome = NatureVoice.cast(&caster, &target, &mut rng);

        assert!(outcome.full_heal);
    }

    #[test]
    fn failure_does_not_grant_full_heal() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(u64::MAX, 0);

        let outcome = NatureVoice.cast(&caster, &target, &mut rng);

        assert!(!outcome.full_heal);
    }
}
