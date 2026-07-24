use crate::core::battle::BattleResult;
use crate::core::turn_queue::TeamId;
use crate::models::battle_dto::{BattleDetail, BattleListItem};
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

pub async fn save_battle(pool: &PgPool, result: &BattleResult) -> anyhow::Result<Uuid> {
    let (winner, loser) = match result.winner_team {
        TeamId::A => (
            &result.character_a_start.name,
            &result.character_b_start.name,
        ),
        TeamId::B => (
            &result.character_b_start.name,
            &result.character_a_start.name,
        ),
    };

    let attacker_hp_at_end = result
        .turns
        .last()
        .map(|t| match t.attacker_team {
            TeamId::A => t.character_a_after.hp,
            TeamId::B => t.character_b_after.hp,
        })
        .unwrap_or(0);

    let full_result = serde_json::to_value(result)?;

    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO battles
            (character_a_name, character_b_name, winner_name, loser_name, attacker_hp_at_end, full_result)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(&result.character_a_start.name)
    .bind(&result.character_b_start.name)
    .bind(winner)
    .bind(loser)
    .bind(attacker_hp_at_end)
    .bind(&full_result)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

pub const SORTABLE_COLUMNS: &[&str] = &[
    "character_a_name",
    "character_b_name",
    "winner_name",
    "loser_name",
    "attacker_hp_at_end",
];

const SEARCH_CONDITION: &str = r#"
    character_a_name ILIKE $1 OR
    character_b_name ILIKE $1 OR
    winner_name ILIKE $1 OR
    loser_name ILIKE $1 OR
    attacker_hp_at_end::text ILIKE $1
"#;

pub async fn list_battles_datatable(
    pool: &PgPool,
    search: &str,
    limit: i64,
    offset: i64,
    order_column: &str,
    order_dir: &str,
) -> anyhow::Result<(Vec<BattleListItem>, i64, i64)> {
    let column = SORTABLE_COLUMNS
        .iter()
        .find(|&&c| c == order_column)
        .copied()
        .unwrap_or("created_at");
    let direction = if order_dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    let query = format!(
        r#"
        SELECT id, character_a_name, character_b_name, winner_name, loser_name, attacker_hp_at_end
        FROM battles
        WHERE {SEARCH_CONDITION}
        ORDER BY {column} {direction}
        LIMIT $2 OFFSET $3
        "#
    );

    let like_pattern = format!("%{search}%");

    let items = sqlx::query_as::<_, BattleListItem>(AssertSqlSafe(query))
        .bind(&like_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM battles")
        .fetch_one(pool)
        .await?;

    let query_filtered = format!("SELECT COUNT(*) FROM battles WHERE {SEARCH_CONDITION}");

    let filtered: (i64,) = sqlx::query_as(AssertSqlSafe(query_filtered))
        .bind(&like_pattern)
        .fetch_one(pool)
        .await?;

    Ok((items, total.0, filtered.0))
}

pub async fn get_battle(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<BattleDetail>> {
    let row = sqlx::query_as::<_, BattleDetail>(
        r#"
        SELECT id, full_result
        FROM battles
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
