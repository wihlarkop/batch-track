mod helper;

use std::net::SocketAddr;
use axum::{routing::get, Router, Json};
use serde_json::{json, Value};
use tracing::info;
use tracing_subscriber;

use crate::helper::server::{fallback, shutdown_signal};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .init();

    let app = Router::new()
        .fallback(fallback)
        .route("/", get(health));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}


async fn health() -> Json<Value> {
    info!("handling req");
    json!({"status": "Success"}).into()
}