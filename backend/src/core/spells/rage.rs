use super::{SpellEffect, SpellOutcome};
use crate::core::character::Character;
use rand::Rng;

pub struct Rage;

impl SpellEffect for Rage {
    fn cast(&self, _caster: &Character, _target: &Character, rng: &mut impl Rng) -> SpellOutcome {
        if rng.gen_bool(1.0 / 5.0) {
            SpellOutcome {
                grants_extra_turn: true,
                ..SpellOutcome::just("Attacker is enraged!")
            }
        } else {
            SpellOutcome::just("Attacker is not angry enough!")
        }
    }
}
