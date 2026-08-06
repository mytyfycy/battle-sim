use crate::db::AppState;
use crate::error::{AppError, AppJson};
use crate::models::user_dto::{AuthResponse, Credentials};
use crate::repository::user_repo;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};
use axum::extract::State;
use axum::http::StatusCode;
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
    AppJson(payload): AppJson<Credentials>,
) -> Result<Json<AuthResponse>, AppError> {
    let payload = normalize_credentials(payload);
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
    AppJson(payload): AppJson<Credentials>,
) -> Result<Json<AuthResponse>, AppError> {
    let payload = normalize_credentials(payload);

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

async fn logout(session: Session) -> Result<StatusCode, AppError> {
    session
        .flush()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Session error: {e}")))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn me(session: Session) -> Result<Json<Option<AuthResponse>>, AppError> {
    let nick: Option<String> = session
        .get(SESSION_USER_NICK_KEY)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Session error: {e}")))?;

    Ok(Json(nick.map(|nick| AuthResponse { nick })))
}

fn normalize_credentials(mut payload: Credentials) -> Credentials {
    payload.nick = payload
        .nick
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    payload
}

fn validate_credentials(payload: &Credentials) -> Result<(), AppError> {
    let nick_len = payload.nick.trim().chars().count();

    if !(3..=32).contains(&nick_len) {
        return Err(AppError::BadRequest(
            "Nick must be between 3 and 32 characters".to_string(),
        ));
    }

    let password_len = payload.password.len();
    if !(8..=64).contains(&password_len) {
        return Err(AppError::BadRequest(
            "Password must be between 8 and 64 characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn creds(nick: &str, password: &str) -> Credentials {
        Credentials {
            nick: nick.to_string(),
            password: password.to_string(),
        }
    }

    fn is_bad_request(result: &Result<(), AppError>) -> bool {
        matches!(result, Err(AppError::BadRequest(_)))
    }

    #[test]
    fn accepts_valid_credentials() {
        let result = validate_credentials(&creds("valid_nick", "password1"));
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_nick_shorter_than_three_chars() {
        let result = validate_credentials(&creds("ab", "password1"));
        assert!(is_bad_request(&result));
    }

    #[test]
    fn accepts_nick_at_minimum_length() {
        let result = validate_credentials(&creds("abc", "password1"));
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_nick_at_maximum_length() {
        let nick = "a".repeat(32);
        let result = validate_credentials(&creds(&nick, "password1"));
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_nick_longer_than_thirty_two_chars() {
        let nick = "a".repeat(33);
        let result = validate_credentials(&creds(&nick, "password1"));
        assert!(is_bad_request(&result));
    }

    #[test]
    fn nick_length_is_trimmed_before_validation() {
        // Trimmed length is 3
        let result = validate_credentials(&creds("  abc  ", "password1"));
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_password_shorter_than_eight_chars() {
        let result = validate_credentials(&creds("valid_nick", "ab12"));
        assert!(is_bad_request(&result));
    }

    #[test]
    fn accepts_password_at_minimum_length() {
        let result = validate_credentials(&creds("valid_nick", "12345678"));
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_password_at_maximum_length() {
        let password = "a".repeat(64);
        let result = validate_credentials(&creds("valid_nick", &password));
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_password_longer_than_sixty_four_chars() {
        let password = "a".repeat(65);
        let result = validate_credentials(&creds("valid_nick", &password));
        assert!(is_bad_request(&result));
    }

    #[test]
    fn normalize_trims_leading_and_trailing_spaces_from_nick() {
        let normalized = normalize_credentials(creds("  Test  ", "password1"));
        assert_eq!(normalized.nick, "Test");
    }

    #[test]
    fn normalize_collapses_internal_whitespace_in_nick() {
        let normalized = normalize_credentials(creds("valid  nick", "password1"));
        assert_eq!(normalized.nick, "valid nick");
    }

    #[test]
    fn normalize_does_not_touch_password() {
        let normalized = normalize_credentials(creds("Test", "  password with spaces  "));
        assert_eq!(normalized.password, "  password with spaces  ")
    }
}
