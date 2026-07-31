mod config;
mod core;
mod db;
mod error;
mod models;
mod repository;
mod routes;

use db::AppState;
use std::net::SocketAddr;
use time::Duration;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
use tower_sessions::session_store::ExpiredDeletion;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let session_pool = sqlx08::postgres::PgPoolOptions::new()
        .connect(&config.database_url)
        .await?;

    let session_store = PostgresStore::new(session_pool);
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_hours(1)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(config.cookie_secure)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let state = AppState { pool };
    let app = routes::router()
        .with_state(state)
        .layer(session_layer)
        .layer(GovernorLayer::new(governor_config));

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Could not bind to port {}: {}", addr, e))?;

    println!("Server is running on: {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    deletion_task.await??;

    Ok(())
}
