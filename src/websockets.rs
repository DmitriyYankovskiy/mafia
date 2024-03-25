use std::sync::Arc;

use serde_json::from_str;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{mpsc::{self, Sender}, Mutex};

use crate::{
    game_state:: {
        GameState,
        game::player::Player,
    },
    AppState,
    PlayerInfo,
};

const BUF: usize = 1000;

pub async fn player(mut ws: WebSocket, state: AppState) {
    let game = state.game;

    let (req_tx, req_rx) = mpsc::channel::<String>(BUF);
    let (res_tx, mut res_rx) = mpsc::channel::<String>(BUF);

    let req_rx = Arc::new(Mutex::new(req_rx));

    while let Some(Ok(msg)) = ws.recv().await {
        if let Ok(msg_str) = msg.to_text() {
            if let Ok(player_info) = from_str::<PlayerInfo>(msg_str) {
                if let GameState::Setup(setup) = &mut *game.lock().await {
                    setup.add_player(Player::new(player_info.name, Sender::clone(&res_tx), req_rx)).await.unwrap();
                }

                break;
            }
        }
    }

    let ws_req = Arc::new(Mutex::new(ws));
    let ws_res = Arc::clone(&ws_req);
    
    tokio::spawn(async move {
        while let Some(msg) = res_rx.recv().await {
            ws_res.lock().await.send(Message::Text(msg)).await.unwrap();
        }
    });

    tokio::spawn(async move {
        while let Some(result_msg) = ws_req.lock().await.recv().await {
            if let Ok(msg) = result_msg {
                if let Ok(msg_str) = msg.to_text() {
                    req_tx.send(msg_str.to_string()).await.unwrap();
                }
            }
        }
    });
}