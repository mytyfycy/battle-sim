mod config;
mod core;
mod db;
mod error;
mod models;
mod repository;
mod routes;

//use axum::http::{HeaderName, Method};
use db::AppState;
use tower_governor::{
    GovernorLayer,
    governor::GovernorConfigBuilder,
    key_extractor::{GlobalKeyExtractor, SmartIpKeyExtractor},
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // let cors = CorsLayer::new()
    //     .allow_origin(
    //         "http://localhost:5173"
    //             .parse::<axum::http::HeaderValue>()
    //             .unwrap(),
    //     )
    //     .allow_methods([Method::GET, Method::POST])
    //     .allow_headers([HeaderName::from_static("content-type")]);
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        //.key_extractor(SmartIpKeyExtractor) // reverse-proxy
        .key_extractor(GlobalKeyExtractor)
        .finish()
        .unwrap();

    let state = AppState { pool };
    let app = routes::router()
        .with_state(state)
        .layer(cors)
        .layer(GovernorLayer::new(governor_config));

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Could not bind to port {}: {}", addr, e))?;

    println!("Server is running on: {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
