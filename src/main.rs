mod helper;

use helper::server::shutdown_signal;
use axum::{routing::get, Router, Json};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(fallback)
        .route("/", get(health));

    let listener = tokio::net::TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap()
}


pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

async fn health() -> Json<Value> {
    json!({"status": "Success"}).into()
}