use crate::db::AppState;
use crate::error::AppError;
use crate::models::user_dto::{AuthResponse, Credentials};
use crate::repository::user_repo;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_sessions::Session;

pub const SESSION_USER_NICK_KEY: &str = "user_nick";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Credentials>,
) -> Result<Json<AuthResponse>, AppError> {
    validate_credentials(&payload)?;

    if user_repo::nick_exists(&state.pool, &payload.nick).await? {
        return Err(AppError::Conflict("Nick is already taken".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Hashing failed: {e}")))?
        .to_string();

    user_repo::create_user(&state.pool, &payload.nick, &password_hash).await?;

    Ok(Json(AuthResponse { nick: payload.nick }))
}

async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<Credentials>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = user_repo::find_by_nick(&state.pool, &payload.nick)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid nick or password".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid stored hash: {e}")))?;

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AppError::Unauthorized(
            "Invalid nick or password".to_string(),
        ));
    }

    session
        .insert(SESSION_USER_NICK_KEY, &user.nick)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Session error: {e}")))?;

    Ok(Json(AuthResponse { nick: user.nick }))
}

async fn logout(session: Session) -> Result<(), AppError> {
    session
        .flush()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Session error: {e}")))?;

    Ok(())
}

async fn me(session: Session) -> Result<Json<AuthResponse>, AppError> {
    let nick: Option<String> = session
        .get(SESSION_USER_NICK_KEY)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Session error: {e}")))?;

    match nick {
        Some(nick) => Ok(Json(AuthResponse { nick })),
        None => Err(AppError::Unauthorized("Not logged in".to_string())),
    }
}

fn validate_credentials(payload: &Credentials) -> Result<(), AppError> {
    let nick_len = payload.nick.trim().chars().count();

    if !(3..=32).contains(&nick_len) {
        return Err(AppError::BadRequest(
            "Nick has to be between 3 and 32 characters".to_string(),
        ));
    }

    if payload.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password has to be at least 8 characters".to_string(),
        ));
    }

    Ok(())
}
