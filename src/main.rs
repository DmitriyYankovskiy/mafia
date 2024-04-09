use axum::{routing::get, Router};
use lobby::{Lobby, State};
use tera::Tera;
use tower_http::services::ServeDir;

use serde::{Serialize, Deserialize};

use std::{net::SocketAddr, sync::Arc};

// ------- mod -------
mod controllers;
mod websockets;
mod lobby;
mod session;
// -------------------


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    name: String
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
    pub lobby: Arc<Lobby>,
}

#[tokio::main]
async fn main() {
    let mut tera = Tera::new("public/**/*.html").unwrap();
    tera.autoescape_on(vec![]);

    let state = AppState {
        tera: Arc::new(tera),
        lobby: Arc::new(Lobby::new()),
    };

    let session = session::Listner::new(Arc::clone(&state.lobby));
    tokio::spawn(session.start());


    let app = Router::new()
        .route("/", get(controllers::index))
        .route("/ws", get(controllers::ws))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}