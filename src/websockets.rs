use std::sync::Arc;

use serde_json::from_str;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{mpsc::{self, Sender}, Mutex};
use futures::{SinkExt, StreamExt};

use crate::{
    lobby::{
        State,
        player::Player,
    },
    AppState,
    PlayerInfo,
};

const BUF: usize = 1000;

pub async fn player(mut ws: WebSocket, state: AppState) {
    let lobby = state.lobby;

    let (req_tx, req_rx) = mpsc::channel::<String>(BUF);
    let (res_tx, mut res_rx) = mpsc::channel::<String>(BUF);

    let req_rx = Mutex::new(req_rx);

    while let Some(Ok(msg)) = ws.recv().await {
        if let Ok(msg_str) = msg.to_text() {
            if let Ok(player_info) = from_str::<PlayerInfo>(msg_str) {
                if let State::Setup = &mut *lobby.state.lock().await {
                    lobby.add_player(Player::new(player_info.name.clone(), Sender::clone(&res_tx), req_rx)).await.unwrap();
                }
                break;
            }
        }
    }

    let (mut ws_req, mut ws_res) = ws.split();

    let _ = tokio::spawn(async move {        
        while let Some(msg) = res_rx.recv().await {
            ws_req.send(Message::Text(msg)).await.unwrap();
        }
    });

    let _ = tokio::spawn(async move {
        while let Some(result_msg) = ws_res.next().await {
            if let Ok(msg) = result_msg {
                if let Ok(msg_str) = msg.to_text() {
                    req_tx.send(msg_str.to_string()).await.unwrap();
                }
            }
        }
    });
}