mod controllers;
mod websockets;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub async fn run(state: super::App) {
    let app = Router::new()
        .route("/", get(controllers::index))
        .route("/ws", get(controllers::ws))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();        
    axum::serve(listener, app).await.unwrap();
}