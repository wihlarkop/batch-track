mod helper;

use std::collections::HashMap;
use std::net::SocketAddr;
use axum::{routing::get, Router, Json};
use axum::http::StatusCode;
use tracing::info;
use tracing_subscriber;
use serde::Serialize;

use crate::helper::server::{fallback, shutdown_signal};

#[derive(Serialize)]
struct HealthResponse {
    service_name: String,
    service_version: String,
    status: HashMap<String, bool>,
}


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


async fn health() -> Json<HealthResponse> {
    let mut status = HashMap::new();

    let response = HealthResponse {
        service_name: "batch tracker".to_string(),
        service_version: "v0.0.1".to_string(),
        status,
    };
    Json(response)
}