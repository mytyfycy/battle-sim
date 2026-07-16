use crate::core::{battle, character};
use crate::db::AppState;
use crate::error::AppError;
use crate::models::battle_dto::{BattleDetail, BattleListItem};
use crate::repository::battle_repo;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use rand::thread_rng;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/battles", post(start_battle).get(list_battles))
        .route("/battles/{id}", get(get_battle))
}

async fn start_battle(
    State(state): State<AppState>,
) -> Result<Json<battle::BattleResult>, AppError> {
    let result = {
        let mut rng = thread_rng();
        let (char_a, char_b) = character::generate_characters(&mut rng);

        battle::simulate_battle(char_a, char_b, &mut rng)?
    };

    battle_repo::save_battle(&state.pool, &result).await?;

    Ok(Json(result))
}

async fn list_battles(
    State(state): State<AppState>,
) -> Result<Json<Vec<BattleListItem>>, AppError> {
    let battles = battle_repo::list_battles(&state.pool).await?;
    Ok(Json(battles))
}

async fn get_battle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BattleDetail>, AppError> {
    let battle = battle_repo::get_battle(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::from(anyhow::anyhow!("Battle not found")))?;

    Ok(Json(battle))
}
