use std::fs;

pub enum FileType {
    Html,
    Css,
    Js,
    Undefined,
}

impl FileType {
    pub fn get_type(path: String) -> FileType {
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

    pub fn to_string(&self) -> String {
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
