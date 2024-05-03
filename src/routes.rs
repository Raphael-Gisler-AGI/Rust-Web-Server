use std::{fs, i8, isize};

use crate::{Response,Status};

type Board = [[bool;20];20];

static mut GAME: Board = [[false;20];20];

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

pub fn reset_game() -> Response {
    set_static_game([[false;20];20]);
    let game = get_static_game();
    let content: String = serde_json::to_string(&game).unwrap();
    Response::new(content, Status::OK)
}

pub fn update_game(body: Option<String>) -> Response {
    let mut game = get_static_game();
    let changed_values: Vec<usize> = serde_json::from_str(&body.unwrap()).unwrap();
    for changed_value in changed_values {
        let x = changed_value % 20;
        let y = changed_value / 20;
        game[y][x] = !game[y][x];
    }
    play_game(&mut game);
    set_static_game(game);
    let content: String = serde_json::to_string(&game).unwrap();
    Response::new(content, Status::OK)
}

const DIRS: [[isize;2];8] = [
    [-1,0],
    [-1,1],
    [0,1],
    [1,1],
    [1,0],
    [1,-1],
    [0,-1],
    [-1,-1]
];

fn play_game(game: &mut Board) {
    let mut to_change: Vec<[usize;2]> = Vec::new();
    for i in 0..game.len() {
        for j in 0..game[i].len() {
            let mut live_neighbors: i8 = 0;
            for dir in DIRS {
                if let Ok(y) = <isize as TryInto<usize>>::try_into(dir[0] + i as isize) {
                    if y >= game.len() {
                        continue;
                    }
                    if let Ok(x) = <isize as TryInto<usize>>::try_into(dir[1] + j as isize) {
                        if x >= game[0].len() {
                            continue;
                        }
                        if game[y][x] {
                            live_neighbors += 1;
                        }
                    }
                }
            }

            if game[i][j] {
                if live_neighbors < 2 || live_neighbors > 3 {
                    to_change.push([i,j]);
                }
            } else if live_neighbors == 3 {
                to_change.push([i,j]);
            }
        }
    }
    for change in to_change {
        game[change[0]][change[1]] = !game[change[0]][change[1]];
    }
}

// Any live cell with fewer than two live neighbors dies, as if by underpopulation.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by overpopulation.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

fn get_static_game() -> Board {
    unsafe {
        return GAME;
    }
}

fn set_static_game(game: Board) {
    unsafe {
        GAME = game;
    }
}

