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
