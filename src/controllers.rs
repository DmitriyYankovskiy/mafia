use axum::{
    extract::{ws::WebSocketUpgrade, State}, 
    response::{Html, IntoResponse}, 
};

use serde_json::json;

use crate::{AppState, websockets};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let tera = state.tera;

    let mut context = tera::Context::from_value(json!({
        "title": "Mafia | Game #001",
        "players" : [
            {"number": 1},
            {"number": 2},
            {"number": 3},
            {"number": 4},
            {"number": 5},
            {"number": 6},
            {"number": 7},
            {"number": 8},
            {"number": 9},
            {"number": 10}
        ]
    })).unwrap();
    context.insert("page", &tera.render("game/index.html", &context).unwrap());
    Html::from(tera.render("layouts/main.html", &context).unwrap())
}

pub async fn ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move|ws| {websockets::player(ws, state)})
}