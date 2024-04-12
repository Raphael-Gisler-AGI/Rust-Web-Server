use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use routing::{Method, Routes, Status, Response};

mod routes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut request_arr = request_line.split(" ");
    let method = request_arr.next().unwrap();
    let path = request_arr.next().unwrap();

    let routes: Routes = Routes::new()
        .add("/", Method::GET, routes::get_index)
        .add("/script.js", Method::GET, routes::get_script)
        .add("/styles.css", Method::GET, routes::get_styles);

    let res: Response = routes.route(path, Method::from_str(method).unwrap());

    let response_line: String = res.status.get_response_line();
    let length: usize = res.length;
    let content: String = res.content;

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", response_line, length, content);

    stream.write_all(response.as_bytes()).unwrap();
}

