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
