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
