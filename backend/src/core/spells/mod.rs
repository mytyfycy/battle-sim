use crate::core::character::Character;
use crate::core::status::StatusEffect;
use rand::Rng;
use serde::Serialize;

pub mod aura;
pub mod critical_hit;
pub mod hard_hit;
pub mod ice_bullet;
pub mod nature_voice;
pub mod rage;
pub mod reincarnation;
pub mod shield;

#[derive(Clone, Copy, Debug)]
pub enum SpellTarget {
    Caster,
    Target,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpellOutcome {
    pub description: String,
    pub extra_damage: i32,
    pub instant_win: bool,
    pub full_heal: bool,
    pub heal_amount: i32,
    pub grants_extra_turn: bool,
    pub defense_increase: i32,
    pub defense_bonus_promised: i32,
    #[serde(skip)]
    pub status_to_apply: Option<(SpellTarget, StatusEffect)>,
}

impl SpellOutcome {
    pub fn just(description: impl Into<String>) -> Self {
        SpellOutcome {
            description: description.into(),
            extra_damage: 0,
            instant_win: false,
            full_heal: false,
            heal_amount: 0,
            grants_extra_turn: false,
            defense_increase: 0,
            defense_bonus_promised: 0,
            status_to_apply: None,
        }
    }
}

pub trait SpellEffect {
    fn cast(&self, caster: &Character, target: &Character, rng: &mut impl Rng) -> SpellOutcome;
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spell {
    HardHit,
    Aura,
    Rage,
    Shield,
    CriticalHit,
    NatureVoice,
    IceBullet,
    Reincarnation,
}

pub fn random_spell(rng: &mut impl Rng) -> Spell {
    const ALL: &[Spell] = &[
        Spell::HardHit,
        Spell::Aura,
        Spell::Rage,
        Spell::Shield,
        Spell::CriticalHit,
        Spell::NatureVoice,
        Spell::IceBullet,
        Spell::Reincarnation,
    ];
    ALL[rng.gen_range(0..ALL.len())]
}

pub fn cast(
    spell: Spell,
    caster: &Character,
    target: &Character,
    rng: &mut impl Rng,
) -> SpellOutcome {
    match spell {
        Spell::HardHit => hard_hit::HardHit.cast(caster, target, rng),
        Spell::Aura => aura::Aura.cast(caster, target, rng),
        Spell::Rage => rage::Rage.cast(caster, target, rng),
        Spell::Shield => shield::Shield.cast(caster, target, rng),
        Spell::CriticalHit => critical_hit::CriticalHit.cast(caster, target, rng),
        Spell::NatureVoice => nature_voice::NatureVoice.cast(caster, target, rng),
        Spell::IceBullet => ice_bullet::IceBullet.cast(caster, target, rng),
        Spell::Reincarnation => reincarnation::Reincarnation.cast(caster, target, rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::character::Character;
    use crate::core::status::StatusEffects;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn dummy_character(spell: Spell) -> Character {
        Character {
            name: "Dummy".to_string(),
            hp: 50,
            max_hp: 50,
            strength: 20,
            defense: 2,
            spell,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn random_spell_can_produce_every_variant() {
        let mut rng = StdRng::seed_from_u64(100);
        let mut seen = [false; 8];

        for _ in 0..2000 {
            let index = match random_spell(&mut rng) {
                Spell::HardHit => 0,
                Spell::Aura => 1,
                Spell::Rage => 2,
                Spell::Shield => 3,
                Spell::CriticalHit => 4,
                Spell::NatureVoice => 5,
                Spell::IceBullet => 6,
                Spell::Reincarnation => 7,
            };
            seen[index] = true;
        }

        assert!(
            seen.iter().all(|&s| s),
            "expected all 8 spell variants to appear, saw: {seen:?}"
        );
    }

    #[test]
    fn cast_dispatches_to_the_matching_spell_implementation() {
        let mut rng = StdRng::seed_from_u64(100);
        let all_spells = [
            Spell::HardHit,
            Spell::Aura,
            Spell::Rage,
            Spell::Shield,
            Spell::CriticalHit,
            Spell::NatureVoice,
            Spell::IceBullet,
            Spell::Reincarnation,
        ];

        for spell in all_spells {
            let caster = dummy_character(spell);
            let target = dummy_character(Spell::HardHit);

            let outcome = cast(spell, &caster, &target, &mut rng);
            assert!(!outcome.description.is_empty());
        }
    }
}
