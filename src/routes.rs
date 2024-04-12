use std::{io::Error, fs};

use super::{Response, Status};

pub fn get_index() -> Response {
    let (content, status) = match get_file_from_public("index.html") {
        Ok(file_content) => (file_content, Status::OK),
        Err(..) => ("File not found".to_string(), Status::NOTFOUND)
    };

    Response::new(content, status)
}

pub fn get_script() -> Response {
    let (content, status) = match get_file_from_public("script.js") {
        Ok(file_content) => (file_content, Status::OK),
        Err(..) => ("File not found".to_string(), Status::NOTFOUND)
    };

    Response::new(content, status)
}

pub fn get_styles() -> Response {
    let (content, status) = match get_file_from_public("styles.css") {
        Ok(file_content) => (file_content, Status::OK),
        Err(..) => ("File not found".to_string(), Status::NOTFOUND)
    };

    Response::new(content, status)
}

fn get_file_from_public(file_name: &str) -> Result<String, Error> {
    fs::read_to_string(format!("public/{}", file_name))
}
