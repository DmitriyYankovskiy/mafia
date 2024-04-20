use std::sync::Arc;

use serde_json::from_str;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{mpsc::{self, Sender}, Mutex};
use futures::{SinkExt, StreamExt};

use crate::{
    internal::lobby::{player::Player, State, message::*},
    App,
    PlayerInfo,
};

const BUF: usize = 1000;

pub async fn player(mut ws: WebSocket, state: App) {
    let lobby = state.lobby;

    let (req_tx, req_rx) = mpsc::channel::<incom::M>(BUF);
    let (res_tx, mut res_rx) = mpsc::channel::<outgo::M>(BUF);

    let req_rx = Mutex::new(req_rx);

    while let Some(Ok(msg)) = ws.recv().await {
        if let Ok(msg_str) = msg.to_text() {
            if let Ok(player_info) = from_str::<PlayerInfo>(msg_str) {
                if let State::Setup = &mut *lobby.state.lock().await {
                    let player = Arc::new(Player::new(player_info.name.clone(), Sender::clone(&res_tx), req_rx));
                    let _ = tokio::spawn(Player::listen(player.clone()));
                    lobby.add_player(player).await.unwrap();
                }
                break;
            }
        }
    }

    let (mut ws_res, mut ws_req) = ws.split();

    let _ = tokio::spawn(async move {        
        while let Some(msg) = res_rx.recv().await {
            ws_res.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.unwrap();
        }
    });

    let _ = tokio::spawn(async move {
        while let Some(result_msg) = ws_req.next().await {
            if let Ok(msg) = result_msg {
                if let Ok(msg_str) = msg.to_text() {
                    req_tx.send(serde_json::from_str(&msg_str).unwrap()).await.unwrap();
                }
            };
        }
    });
}