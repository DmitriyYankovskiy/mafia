use serde_json::from_str;
use serde::Deserialize;

use axum::extract::ws::WebSocket;
use tokio::sync::mpsc;

use crate::{game::{Player, GameState}, AppState};

const BUF: usize = 1000;


#[derive(Debug, Clone, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
}

pub async fn player(ws: WebSocket, state: AppState) {
    let mut game = state.game.lock().await;

    let (tx, mut rx) = mpsc::channel::<String>(BUF);
    let mut name: Option<String> = None;

    if let Some(msg_string) = rx.recv().await {
        let player_info: PlayerInfo = from_str(&msg_string).unwrap();
        if let GameState::Setup(setup) = &mut (*game) {
            setup.add_player(Player::new(player_info.name, tx));
        }
    }
}