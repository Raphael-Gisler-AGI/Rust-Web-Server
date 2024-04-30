use std::fs;

use crate::{Response,Status};

static mut GAME: [[bool;20];20] = [[false;20];20];

pub fn get_index() -> Response {
    get_file_from_public("index.html")
}

pub fn get_script() -> Response {
    get_file_from_public("script.js")
}

pub fn get_styles() -> Response {
    get_file_from_public("styles.css")
}

fn get_file_from_public(file_name: &str) -> Response {
    let file = fs::read_to_string(format!("public/{}", file_name));
    let (content, status) = match file {
        Ok(file_content) => (file_content, Status::OK),
        Err(..) => ("File not found".to_string(), Status::NOTFOUND)
    };

    Response::new(content, status)
}

pub fn get_game() -> Response {
    let game = get_static_game();
    let content: String = serde_json::to_string(&game).unwrap();

    Response::new(content, Status::OK)
}

pub fn update_game(body: Option<String>) -> Response {
    let response_message = format!("from update_game {}", body.unwrap());
    Response::new(response_message, Status::OK)
}

fn get_static_game() -> [[bool;20];20] {
    unsafe {
        return GAME;
    }
}

