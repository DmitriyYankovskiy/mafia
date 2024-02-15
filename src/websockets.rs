use std::sync::Arc;

use serde_json::from_str;
use serde::Deserialize;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{broadcast, mpsc, Mutex};

use crate::{game::{GameState, Player}, AppState, PlayerInfo};

const BUF: usize = 1000;

pub async fn player(mut ws: WebSocket, state: AppState) {
    let mut game = state.game.lock().await;

    let (req_tx, mut req_rx) = mpsc::channel::<String>(BUF);
    let (res_tx, mut res_rx) = mpsc::channel::<String>(BUF);
    let mut name: Option<String> = None;

    let req_rx = Arc::new(Mutex::new(req_rx));

    while let Some(Ok(msg)) = ws.recv().await {
        if let Ok(msg_str) = msg.to_text() {
            if let Ok(player_info) = from_str::<PlayerInfo>(msg_str) {
                if let GameState::Setup(setup) = &mut (*game) {
                    setup.add_player(Player::new(player_info.name, res_tx.clone(), req_rx));
                }

                break;
            }
        }
    }

    let ws_req = Arc::new(Mutex::new(ws));
    let ws_res = ws_req.clone();
    
    tokio::spawn(async move {
        while let Some(msg) = res_rx.recv().await {
            let _ = ws_res.lock().await.send(Message::Text(msg)).await;
        }
    });

    tokio::spawn(async move {
        while let Some(result_msg) = ws_req.lock().await.recv().await {
            if let Ok(msg) = result_msg {
                if let Ok(msg_str) = msg.to_text() {
                    let _ = req_tx.send(msg_str.to_string()).await;
                }
            }
        }
    });
}