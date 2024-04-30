use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

use routing::{Routes, Status, Response, Request};

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
        .get("/game", routes::get_game)
        .patch("/game", routes::update_game)
        .get("/", routes::get_index)
        .get("/script.js", routes::get_script)
        .get("/styles.css", routes::get_styles);

    let res: Response = routes.route(request);

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
        res.status.get_response_line(),
        res.length,
        res.content
    );

    stream.write_all(response.as_bytes()).unwrap();
}

