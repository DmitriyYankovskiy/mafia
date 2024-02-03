use axum::{routing::get, Error, Router};
use handlebars::{HelperDef, HelperResult, JsonRender, Handlebars, JsonValue};

use reqwest::Response;

use serde::{Serialize, Deserialize};
use serde_json::json;

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

fn hbs_init(hbs: &mut Handlebars) {
    hbs.register_helper("partial", Box::new(
        |h: &handlebars::Helper, hbs: &Handlebars, ctx: &handlebars::Context, rc: &mut handlebars::RenderContext, out: &mut dyn handlebars::Output| -> HelperResult {
            let name =
            h.param(0).ok_or(handlebars::RenderErrorReason::ParamNotFoundForIndex("closure-helper", 0))?;

            out.write(file::file_to_string(name.value().render()).as_str())?;
            Ok(())
        }
    ));
}

#[derive(Clone)]
pub struct AppState<'a> {
    pub hbs: Arc<Handlebars<'a>>,
}

#[tokio::main]
async fn main() {
    let mut hbs: Handlebars<'_> = Handlebars::new();
    hbs_init(&mut hbs);

    let state = AppState {
        hbs: Arc::new(hbs),
    };

    let app = Router::new()
        .route("/", get(controllers::index))
        .route("/public/*path", get(controllers::static_files))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}