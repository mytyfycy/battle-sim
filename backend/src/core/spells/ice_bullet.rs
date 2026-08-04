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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::character::Character;
    use crate::core::spells::Spell;
    use crate::core::status::StatusEffects;
    use rand::rngs::mock::StepRng;

    fn dummy_character() -> Character {
        Character {
            name: "Dummy".to_string(),
            hp: 50,
            max_hp: 50,
            strength: 20,
            defense: 2,
            spell: Spell::IceBullet,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn always_deals_eight_damage() {
        let caster = dummy_character();
        let target = dummy_character();
        let mut rng = StepRng::new(0, 0);

        let outcome = IceBullet.cast(&caster, &target, &mut rng);

        assert_eq!(outcome.extra_damage, 8);
    }
}
