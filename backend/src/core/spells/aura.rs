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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::character::Character;
    use crate::core::spells::Spell;
    use crate::core::status::{StatusEffects, StatusKind};
    use rand::rngs::mock::StepRng;

    fn dummy_character() -> Character {
        Character {
            name: "Dummy".to_string(),
            hp: 50,
            max_hp: 50,
            strength: 20,
            defense: 2,
            spell: Spell::Aura,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn always_promises_five_defense_and_targets_the_caster() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(0, 0);

        let outcome = Aura.cast(&caster, &target, &mut rng);

        assert_eq!(outcome.defense_bonus_promised, 5);

        match outcome.status_to_apply {
            Some((SpellTarget::Caster, effect)) => match effect.kind {
                StatusKind::ExtraDefenseAura { amount } => assert_eq!(amount, 5),
            },
            _ => panic!("Aura should apply a status effect to the caster"),
        }
    }
}
