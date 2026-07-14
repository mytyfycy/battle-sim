mod battles;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(battles::router())
}
