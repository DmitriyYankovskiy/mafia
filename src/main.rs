use axum::{routing::get, Error, Router};
use game::GameState;
use tera::Tera;
use tower_http::services::ServeDir;

use serde::{Serialize, Deserialize};

use std::{net::SocketAddr, sync::Arc};

// ------- mod -------
mod player;
mod file;
mod controllers;
mod websockets;
mod game;
mod game_loop;
mod characters;
// -------------------


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    name: String
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
    pub game: GameState,
}


#[tokio::main]
async fn main() {
    let mut tera = Tera::new("public/**/*.html").unwrap();
    tera.autoescape_on(vec![]);

    let state = AppState {
        tera: Arc::new(tera),
        game: GameState::new(),
    };

    let app = Router::new()
        .route("/", get(controllers::index))
        .route("/ws", get(controllers::ws))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}