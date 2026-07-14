use axum::Router;

mod core;
mod routes;

#[tokio::main]
async fn main() {
    let app: Router = routes::router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind to port 3000");

    axum::serve(listener, app).await.unwrap();
}
