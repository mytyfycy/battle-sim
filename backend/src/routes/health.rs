use crate::db::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(|| async { "OK" }))
}
