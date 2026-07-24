use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct DataTableParams {
    pub draw: i64,
    pub start: i64,
    pub length: i64,
    #[serde(rename = "search[value]")]
    pub search_value: Option<String>,
    #[serde(rename = "order[0][column]")]
    pub order_column_index: Option<usize>,
    #[serde(rename = "order[0][dir]")]
    pub order_dir: Option<String>,
}

#[derive(Serialize)]
pub struct DataTableResponse {
    pub draw: i64,
    #[serde(rename = "recordsTotal")]
    pub records_total: i64,
    #[serde(rename = "recordsFiltered")]
    pub records_filtered: i64,
    pub data: Vec<BattleListItem>,
}
