use axum::{routing::get, Error, Router};
use controllers::ws_game;
use tera::Tera;
use tower_http::services::ServeDir;
use reqwest::Response;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use std::{net::SocketAddr, ops::Deref, sync::Arc};
pub use std::{fs, sync::Mutex, collections::HashMap};

mod file;
mod controllers;
mod game;
// mod game_loop;
mod characters;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo<T> {
    name: String,
    info: T,
}

fn loop_filter<'a, 'b>(val: &'a Value, args: &'b HashMap<String, Value>) -> tera::Result<Value> {
    let string = val.as_str().unwrap_or("").to_string();
    let mut ans = "".to_string();

    let count = match args.get(&"count".to_string()) {
        Some(val) => val.as_u64().unwrap_or(0),
        None => 0,
    };
    for i in 0..count {
        ans.push_str(&string);
    }
    tera::Result::Ok(Value::String(ans))
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
}

#[tokio::main]
async fn main() {
    let mut tera = Tera::new("public/**/*.*").unwrap();
    tera.autoescape_on(vec![]);
    tera.register_filter("loop", loop_filter);

    let state = AppState {
        tera: Arc::new(tera),
    };

    let app = Router::new()
        .route("/", get(controllers::index))
        .route("/ws", ws_game)
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}