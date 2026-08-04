use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct HardHit;

impl SpellEffect for HardHit {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        let damage = rng.gen_range(5..=10);

        SpellOutcome {
            extra_damage: damage,
            ..SpellOutcome::just(format!("Hard hit dealt {damage}HP!"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::character::Character;
    use crate::core::spells::Spell;
    use crate::core::status::StatusEffects;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn dummy_character() -> Character {
        Character {
            name: "Dummy".to_string(),
            hp: 50,
            max_hp: 50,
            strength: 20,
            defense: 2,
            spell: Spell::HardHit,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn extra_damage_is_within_expected_range() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StdRng::seed_from_u64(100);

        for _ in 0..500 {
            let outcome = HardHit.cast(&caster, &target, &mut rng);
            assert!((5..=10).contains(&outcome.extra_damage));
            assert!(!outcome.instant_win);
            assert!(!outcome.full_heal);
        }
    }
}
