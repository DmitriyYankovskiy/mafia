use axum::{routing::get, Error, Router};
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

// fn hbs_init(hbs: &mut Handlebars) {
//     hbs.register_helper("partial", Box::new(
//         |h: &handlebars::Helper, hbs: &Handlebars, ctx: &handlebars::Context, rc: &mut handlebars::RenderContext, out: &mut dyn handlebars::Output| -> HelperResult {
//             let name =
//             h.param(0).ok_or(handlebars::RenderErrorReason::ParamNotFoundForIndex("closure-helper", 0))?;

//             out.write(file::file_to_string(name.value().render()).as_str())?;
//             Ok(())
//         }
//     ));
// }

// fn loop_filter(v: &Value, hm: &HashMap<String, Value>) -> Result<Value, tera::Error> {
//     let string = match v.as_str() {
//         Some(s) => s,
//         None => "",
//     }.to_string();
//     let mut ans = "".to_string();
//     for i in 0..3 {
//         ans.push_str(&string)
//     }
//     Result::Ok(Value::String(ans))
// }

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
}

#[tokio::main]
async fn main() {
    let mut tera = Tera::new("public/**/*.*").unwrap();
    tera.autoescape_on(vec![]);

    let state = AppState {
        tera: Arc::new(tera),
    };

    let app = Router::new()
        .route("/", get(controllers::index))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}