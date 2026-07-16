mod config;
mod core;
mod db;
mod error;
mod models;
mod repository;
mod routes;

use db::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { pool };
    let app = routes::router().with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Could not bind to port {}: {}", addr, e))?;

    println!("Server is running on: {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
