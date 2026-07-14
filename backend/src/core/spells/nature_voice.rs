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
