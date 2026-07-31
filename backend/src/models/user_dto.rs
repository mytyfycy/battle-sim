use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Credentials {
    pub nick: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub nick: String,
}

#[derive(sqlx::FromRow)]
pub struct UserRecord {
    pub nick: String,
    pub password_hash: String,
}
