use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

use routing::{Method, Routes, Status, Response, Request};

mod routes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    let _ = stream.read(&mut buffer).unwrap();
    let request = Request::new(&buffer);

    println!("{:?}", request.body);

    let routes: Routes = Routes::new()
        .add("/game", Method::GET, routes::get_game)
        .add("/game", Method::UPDATE, routes::update_game)
        .add("/", Method::GET, routes::get_index)
        .add("/script.js", Method::GET, routes::get_script)
        .add("/styles.css", Method::GET, routes::get_styles);

    let res: Response = routes.route(
        &request.request_line.path,
        request.request_line.method
    );

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
        res.status.get_response_line(),
        res.length,
        res.content
    );

    stream.write_all(response.as_bytes()).unwrap();
}

