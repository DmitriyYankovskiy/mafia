use actix_web::{web, HttpResponse};
use handlebars::{Handlebars, JsonValue};
use std::fs;

pub enum FileType {
    Html,
    Css,
    Js,
    Undefined,
}

impl FileType {
    fn get_type(path: String) -> FileType {
        let parts: Vec<&str> = path.split(".").collect();
        let ext = parts[parts.len() - 1];
        if *ext == *"html" {
            return FileType::Html;
        } else if *ext == *"css" {
            return FileType::Css;
        } else if *ext == *"js" {
            return FileType::Js;
        }
        FileType::Undefined
    }

    fn to_string(&self) -> String {
        match self {
            FileType::Html => "text/html".to_string(),
            FileType::Css => "text/css".to_string(),
            FileType::Js => "text/js".to_string(),
            FileType::Undefined => "text/undefined".to_string()
        }
    }
}


pub fn file_to_string(path: String) -> String {
    match fs::read_to_string(format!("www/{}", path)) {
        Ok(file) => file,
        Err(..) => "Error 404".to_string(),
    }
} 

pub fn file_response(path: String) -> HttpResponse {
    match fs::read_to_string(format!("{}", path)) {
        Ok(file) => HttpResponse::Ok().content_type().body(file),
        Err(..) => HttpResponse::Forbidden().body("404"),
    }
}

fn file_in_layout_response(layout_path: String, options: JsonValue, hbs_data: web::Data<Handlebars<'_>>) -> HttpResponse {
    match fs::read_to_string(format!("www/layouts/{}.html", layout_path)) {
        Ok(file) => HttpResponse::Ok().content_type(FileType::Html.to_string())
        .body(match hbs_data.render_template(&file, &options) {
            Ok(file) => file,
            Err(..) => return HttpResponse::Forbidden().body("500"),
        }),
        Err(..) => HttpResponse::Forbidden().body("404"),
    }
}