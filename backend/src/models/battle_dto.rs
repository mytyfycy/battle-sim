use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct BattleListItem {
    pub id: Uuid,
    pub character_a_name: String,
    pub character_b_name: String,
    pub winner_name: String,
    pub loser_name: String,
    pub attacker_hp_at_end: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct BattleDetail {
    pub id: Uuid,
    pub full_result: serde_json::Value,
}
