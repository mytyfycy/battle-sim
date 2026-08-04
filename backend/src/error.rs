use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    Conflict(String),
    BadRequest(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": msg }))).into_response()
            }
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, Json(json!({ "error": msg }))).into_response()
            }
            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, Json(json!({ "error": msg }))).into_response()
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response()
            }
            AppError::Internal(err) => {
                eprintln!("Request error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal server error" })),
                )
                    .into_response()
            }
        }
    }
}

// Allows use of the '?' operator on any error that can be converted to anyhow::Error
// (ex. sqlx::Error)
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}

pub struct AppJson<T>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(value)) => Ok(AppJson(value)),
            Err(_rejection) => Err(AppError::BadRequest("Invalid request body".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn body_json(response: Response) -> serde_json::Value {
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn not_found_maps_to_404() {
        let response = AppError::NotFound("missing".to_string()).into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = body_json(response).await;
        assert_eq!(body["error"], "missing");
    }

    #[tokio::test]
    async fn unauthorized_maps_to_401() {
        let response = AppError::Unauthorized("unauthorized".to_string()).into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body = body_json(response).await;
        assert_eq!(body["error"], "unauthorized");
    }

    #[tokio::test]
    async fn conflict_maps_to_409() {
        let response = AppError::Conflict("taken".to_string()).into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);

        let body = body_json(response).await;
        assert_eq!(body["error"], "taken");
    }

    #[tokio::test]
    async fn bad_request_maps_to_400() {
        let response = AppError::BadRequest("bad".to_string()).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = body_json(response).await;
        assert_eq!(body["error"], "bad");
    }

    #[tokio::test]
    async fn internal_maps_to_500_and_hides_the_real_error_message() {
        let response = AppError::Internal(anyhow::anyhow!("db error")).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = body_json(response).await;
        assert_eq!(body["error"], "Internal server error");
    }

    #[test]
    fn any_convertible_error_becomes_an_internal_app_error() {
        let io_err = std::io::Error::other("io error");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Internal(_)));
    }
}
