use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State}, 
    response::{Html, IntoResponse, Response}, 
    http::StatusCode
};

use serde_json::json;

use crate::{file, AppState, websockets};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let tera = state.tera;

    let mut context = tera::Context::new();
    context.insert("title", &"Aboba".to_string());
    context.insert("page", &tera.render("game/index.html", &context).unwrap());
    Html::from(tera.render("layouts/main.html", &context).unwrap())
}

pub async fn ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move|ws| {websockets::player(ws, state)})
}