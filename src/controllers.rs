use std::sync::Arc;

use axum::{extract::{Path, State}, response::{Html, IntoResponse, Response}, http::StatusCode};
use serde_json::json;
use crate::{file, AppState};

pub async fn index(State(state): State<AppState>) -> Response {
    let tera = state.tera;

    let mut context = tera::Context::from_value(json!({
        "title": "Aboba",
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
    Html::from(tera.render("layouts/main.html", &context).unwrap()).into_response()
    //file::file_in_layout_response("main".to_string(), json!({"title": "Mafia game | gamecode", "page": "game/index.html"}), hbs)
}

// pub async fn static_files(Path(path): Path<String>) -> Response {
//     file::file_response(format!("public/{}", path).to_string())
// }

// #[get("/connect")]
// pub async fn connect(hbs_data: web::Data<Handlebars<'_>>, json: web::Json<PlayerInfo<String>>) -> impl Responder {
//     let mut game = game_data.lock().unwrap();
//     match game.add_player(json.0) {
//         Ok(id) => {
//             file_in_layout_response("main".to_string(), json!({"title": "Game", "page": "game/index.html"}), hbs_data)
//         },
//         Err(..) => {
//             HttpResponse::InternalServerError().body("character not found")
//         }
//     }
// }
