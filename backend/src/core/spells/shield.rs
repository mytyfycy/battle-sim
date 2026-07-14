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
