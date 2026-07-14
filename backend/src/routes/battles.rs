use crate::core::{battle, character};
use axum::{Json, Router, routing::post};
use rand::thread_rng;

pub fn router() -> Router {
    Router::new().route("/battles", post(start_battle))
}

async fn start_battle() -> Result<Json<battle::BattleResult>, axum::http::StatusCode> {
    let mut rng = thread_rng();
    let (char_a, char_b) = character::generate_characters(&mut rng);

    battle::simulate_battle(char_a, char_b, &mut rng)
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}
