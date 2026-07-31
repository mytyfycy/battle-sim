use crate::models::user_dto::UserRecord;
use sqlx::PgPool;

pub async fn nick_exists(pool: &PgPool, nick: &str) -> anyhow::Result<bool> {
    let row: Option<(i32,)> = sqlx::query_as("SELECT 1 FROM users WHERE nick = $1")
        .bind(nick)
        .fetch_optional(pool)
        .await?;

    Ok(row.is_some())
}

pub async fn find_by_nick(pool: &PgPool, nick: &str) -> anyhow::Result<Option<UserRecord>> {
    let user =
        sqlx::query_as::<_, UserRecord>("SELECT nick, password_hash FROM users WHERE nick = $1")
            .bind(nick)
            .fetch_optional(pool)
            .await?;

    Ok(user)
}

pub async fn create_user(pool: &PgPool, nick: &str, password_hash: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO users (nick, password_hash) VALUES ($1, $2)")
        .bind(nick)
        .bind(password_hash)
        .execute(pool)
        .await?;

    Ok(())
}
