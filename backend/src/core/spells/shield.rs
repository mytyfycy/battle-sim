use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct Shield;

impl SpellEffect for Shield {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        let amount = rng.gen_range(1..=3);

        SpellOutcome {
            defense_increase: amount,
            ..SpellOutcome::just(format!("Defense up by {amount}!"))
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
            spell: Spell::Shield,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn defense_increase_is_within_expected_range() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StdRng::seed_from_u64(300);

        for _ in 0..500 {
            let outcome = Shield.cast(&caster, &target, &mut rng);
            assert!((1..=3).contains(&outcome.defense_increase));
        }
    }
}
