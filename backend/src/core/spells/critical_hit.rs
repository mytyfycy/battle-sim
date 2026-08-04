use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct CriticalHit;

impl SpellEffect for CriticalHit {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        if rng.gen_bool(1.0 / 10.0) {
            SpellOutcome {
                instant_win: true,
                ..SpellOutcome::just("Attacker hit a critical!")
            }
        } else {
            SpellOutcome::just("Attacker missed a critical!")
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
            spell: Spell::Aura,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn success_grants_instant_win() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(0, 0); // gen_bool always true

        let outcome = CriticalHit.cast(&caster, &target, &mut rng);

        assert!(outcome.instant_win);
    }

    #[test]
    fn failure_does_not_grant_instant_win() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(u64::MAX, 0); // gen_bool always false

        let outcome = CriticalHit.cast(&caster, &target, &mut rng);

        assert!(!outcome.instant_win);
        assert_eq!(outcome.extra_damage, 0);
    }
}
