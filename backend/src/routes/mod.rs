mod battles;

use crate::db::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().merge(battles::router())
}
