use std::sync::Arc;

use axum::{extract::{Path, State}, response::{Html, IntoResponse, Response}, http::StatusCode};
use serde_json::json;
use crate::{file, AppState};

pub async fn index(State(state): State<AppState>) -> Response {
    let tera = state.tera;

    let mut context = tera::Context::new();
    context.insert("title", &"Aboba".to_string());
    context.insert("page", &tera.render("game/index.html", &context).unwrap());
    Html::from(tera.render("layouts/main.html", &context).unwrap()).into_response()
}

pub async fn ws_game() {
    let (mut tx, mut rx) = ws.split();
    while let Some(Ok(msg)) = rx.next().await {
        tx.send(Message::Text(msg.to_string())).await.unwrap();
    } 
}