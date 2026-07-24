use crate::core::{battle, character};
use crate::db::AppState;
use crate::error::AppError;
use crate::models::battle_dto::{BattleDetail, DataTableParams, DataTableResponse};
use crate::repository::battle_repo;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use rand::thread_rng;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/battles", post(start_battle).get(list_battles_datatable))
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

async fn list_battles_datatable(
    State(state): State<AppState>,
    Query(params): Query<DataTableParams>,
) -> Result<Json<DataTableResponse>, AppError> {
    let order_column = params
        .order_column_index
        .and_then(|i| battle_repo::SORTABLE_COLUMNS.get(i))
        .copied()
        .unwrap_or("created_at");
    let order_dir = params.order_dir.as_deref().unwrap_or("desc");
    let search = params.search_value.unwrap_or_default();

    let (items, total, filtered) = battle_repo::list_battles_datatable(
        &state.pool,
        &search,
        params.length,
        params.start,
        order_column,
        order_dir,
    )
    .await?;

    Ok(Json(DataTableResponse {
        draw: params.draw,
        records_total: total,
        records_filtered: filtered,
        data: items,
    }))
}

async fn get_battle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BattleDetail>, AppError> {
    let battle = battle_repo::get_battle(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Battle not found".to_string()))?;

    Ok(Json(battle))
}
