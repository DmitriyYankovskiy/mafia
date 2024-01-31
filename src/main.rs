use actix_web::{get, web::{self, Json}, App, HttpResponse, HttpServer, Responder, HttpRequest};
use handlebars::{HelperDef, HelperResult, JsonRender, Handlebars, JsonValue};

use reqwest::Response;

use serde::{Serialize, Deserialize};
use serde_json::json;

use std::ops::Deref;
pub use std::{fs, io::Result, sync::Mutex, collections::HashMap};

mod file;
mod controllers;
mod game
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

#[actix_web::main]
async fn main() -> Result<()> {
    let mut hbs: Handlebars<'_> = Handlebars::new();
    hbs_init(&mut hbs);

    let hbs_data: web::Data<Handlebars<'_>> = web::Data::new(hbs);
    HttpServer::new(move || 
        App::new()
        .app_data(hbs_data.clone())
        .service(controllers::index)
    )    
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}