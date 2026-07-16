use crate::core::battle::BattleResult;
use crate::core::turn_queue::TeamId;
use crate::models::battle_dto::{BattleDetail, BattleListItem};
use sqlx::PgPool;
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

pub async fn list_battles(pool: &PgPool) -> anyhow::Result<Vec<BattleListItem>> {
    let rows = sqlx::query_as::<_, BattleListItem>(
        r#"
        SELECT id, character_a_name, character_b_name, winner_name, loser_name, attacker_hp_at_end
        FROM battles
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_battle(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<BattleDetail>> {
    let row = sqlx::query_as::<_, BattleDetail>(
        r#"
        SELECT id, full_result
        FROM battles
        WHERE  id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
