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

#[cfg(test)]
mod tests {

    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn make_character(hp: i32, max_hp: i32) -> Character {
        Character {
            name: "Test".to_string(),
            hp,
            max_hp,
            strength: 20,
            defense: 2,
            spell: Spell::HardHit,
            status_list: StatusEffects::new(),
        }
    }

    #[test]
    fn is_alive_true_when_hp_positive() {
        let character = make_character(1, 100);
        assert!(character.is_alive());
    }

    #[test]
    fn is_alive_false_when_hp_zero() {
        let character = make_character(0, 100);
        assert!(!character.is_alive());
    }

    #[test]
    fn take_damage_reduces_hp() {
        let mut character = make_character(50, 100);
        character.take_damage(20);
        assert_eq!(character.hp, 30);
    }

    #[test]
    fn take_damage_never_goes_below_zero() {
        let mut character = make_character(10, 100);
        character.take_damage(999);
        assert_eq!(character.hp, 0);
    }

    #[test]
    fn heal_increases_hp() {
        let mut character = make_character(50, 100);
        character.heal(20);
        assert_eq!(character.hp, 70);
    }

    #[test]
    fn heal_is_capped_at_max_hp() {
        let mut character = make_character(90, 100);
        character.heal(50);
        assert_eq!(character.hp, 100);
    }

    #[test]
    fn heal_to_full_sets_hp_to_max() {
        let mut character = make_character(1, 100);
        character.heal_to_full();
        assert_eq!(character.hp, 100);
    }

    #[test]
    fn increase_defense_adds_to_current_defense() {
        let mut character = make_character(50, 100);
        character.increase_defense(3);
        assert_eq!(character.defense, 5);
    }

    #[test]
    fn generate_character_stats_are_within_expected_ranges() {
        let mut rng = StdRng::seed_from_u64(5000);

        for _ in 0..500 {
            let character = generate_character("Nox".to_string(), &mut rng);

            assert!((50..=100).contains(&character.max_hp));
            assert_eq!(character.hp, character.max_hp);
            assert!((15..=25).contains(&character.strength));
            assert!((1..=5).contains(&character.defense));
            assert!(character.status_list.is_empty());
        }
    }

    #[test]
    fn generate_characters_never_produces_duplicate_names() {
        let mut rng = StdRng::seed_from_u64(1000);

        for _ in 0..500 {
            let (a, b) = generate_characters(&mut rng);
            assert_ne!(a.name, b.name);
        }
    }
}
