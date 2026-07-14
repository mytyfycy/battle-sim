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
