use actix_web::{get, web, Responder};
use handlebars::Handlebars;
use serde_json::json;
use crate::file;

#[get("/")]
pub async fn index(hbs_data: web::Data<Handlebars<'_>>) -> impl Responder {
    file::file_in_layout_response("main".to_string(), json!({"title": "Game", "page": "game/index.html"}), hbs_data)
}

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
