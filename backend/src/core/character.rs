use crate::core::spells::{self, Spell};
use crate::core::status::StatusEffects;
use rand::Rng;
use serde::Serialize;

const NAMES: &[&str] = &[
    "Nox", "Stryx", "Rex", "Aethel", "Zeal", "Valur", "Xyron", "Ryxon",
];

#[derive(Serialize, Clone, Debug)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub strength: i32,
    pub defense: i32,
    pub spell: Spell,
    pub status_list: StatusEffects,
}

impl Character {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn heal_to_full(&mut self) {
        self.hp = self.max_hp;
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    pub fn increase_defense(&mut self, amount: i32) {
        self.defense += amount;
    }
}

pub fn generate_character(name: String, rng: &mut impl Rng) -> Character {
    let max_hp = rng.gen_range(50..=100);

    Character {
        name,
        hp: max_hp,
        max_hp,
        strength: rng.gen_range(15..=25),
        defense: rng.gen_range(1..=5),
        spell: spells::random_spell(rng),
        status_list: StatusEffects::new(),
    }
}

pub fn generate_characters(rng: &mut impl Rng) -> (Character, Character) {
    let name_a = random_name(rng);
    let mut name_b = random_name(rng);
    while name_b == name_a {
        name_b = random_name(rng);
    }

    (
        generate_character(name_a, rng),
        generate_character(name_b, rng),
    )
}

fn random_name(rng: &mut impl Rng) -> String {
    NAMES[rng.gen_range(0..NAMES.len())].to_string()
}
