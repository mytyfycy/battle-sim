use super::{SpellEffect, SpellOutcome, SpellTarget};
use crate::core::character::Character;
use crate::core::status::{StatusEffect, StatusKind};
use rand::Rng;

pub struct Aura;

impl SpellEffect for Aura {
    fn cast(&self, _caster: &Character, _target: &Character, _rng: &mut impl Rng) -> SpellOutcome {
        const AMOUNT: i32 = 5;
        let status = StatusEffect::once(StatusKind::ExtraDefenseAura { amount: AMOUNT });

        SpellOutcome {
            defense_bonus_promised: AMOUNT,
            status_to_apply: Some((SpellTarget::Caster, status)),
            ..SpellOutcome::just(format!("Next defense just got stronger!"))
        }
    }
}
