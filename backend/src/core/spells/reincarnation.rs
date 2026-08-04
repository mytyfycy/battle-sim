use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct Reincarnation;

impl SpellEffect for Reincarnation {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        let heal = rng.gen_range(5..=10);

        SpellOutcome {
            heal_amount: heal,
            ..SpellOutcome::just(format!("Reincarnation just healed {}HP!", heal))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::character::Character;
    use crate::core::spells::Spell;
    use crate::core::status::StatusEffects;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn dummy_character() -> Character {
        Character {
            name: "Dummy".to_string(),
            hp: 50,
            max_hp: 50,
            strength: 20,
            defense: 2,
            spell: Spell::Reincarnation,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn heal_amount_is_within_expected_range() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StdRng::seed_from_u64(200);

        for _ in 0..500 {
            let outcome = Reincarnation.cast(&caster, &target, &mut rng);
            assert!((5..=10).contains(&outcome.heal_amount));
            assert!(!outcome.full_heal);
            assert_eq!(outcome.extra_damage, 0);
        }
    }
}
