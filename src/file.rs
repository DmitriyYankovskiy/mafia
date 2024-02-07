use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use std::{fs, sync::Arc};

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
            FileType::Undefined => "text/undefined".to_string(),
        }
    }
}

pub fn file_to_string(path: String) -> String {
    match fs::read_to_string(format!("public/{}", path)) {
        Ok(file) => file,
        Err(..) => "Error 404".to_string(),
    }
}

// pub fn file_response(path: String) -> Response {
//     match fs::read_to_string(format!("{}", path)) {
//         Ok(file) => file.into_response().,
//         Err(..) => StatusCode::FORBIDDEN.into_response(),
//     }
// }

// pub fn file_in_layout_response(layout_path: String, options: JsonValue, hbs_data: Arc<Handlebars<'_>>) -> Response {
//     match fs::read_to_string(format!("public/layouts/{}.html", layout_path)) {
//         Ok(file) => {
//             match hbs_data.render_template(&file, &options) {
//                 Ok(file) => Html::from(file).into_response(),
//                 Err(..) => return StatusCode::FORBIDDEN.into_response(),
//             }
//         },
//         Err(..) => StatusCode::FORBIDDEN.into_response(),
//     }
// }
