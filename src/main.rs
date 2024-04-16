mod server;
mod internal;

use std::sync::Arc;

use tera::Tera;
use serde::{Serialize, Deserialize};

use internal::{
    lobby::{Lobby, State},
    console::Console,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    name: String
}

#[derive(Clone)]
pub struct App {
    pub tera: Arc<Tera>,
    pub lobby: Arc<Lobby>,
}

#[tokio::main]
async fn main() {
    let mut tera = Tera::new("public/**/*.html").unwrap();
    tera.autoescape_on(vec![]);

    let app = App {
        tera: Arc::new(tera),
        lobby: Arc::new(Lobby::new()),
    };

    let console = Console::new(Arc::clone(&app.lobby));
    tokio::spawn(console.start());
    server::run(app).await;
}