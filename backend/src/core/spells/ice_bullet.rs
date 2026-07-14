use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct IceBullet;

impl SpellEffect for IceBullet {
    fn cast(&self, _caster: &Character, _target: &Character, _rng: &mut impl Rng) -> SpellOutcome {
        const DAMAGE: i32 = 8;

        SpellOutcome {
            extra_damage: DAMAGE,
            ..SpellOutcome::just(format!("Ice bullet dealt {DAMAGE}HP!"))
        }
    }
}
