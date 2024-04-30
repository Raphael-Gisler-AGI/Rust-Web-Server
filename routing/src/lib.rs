use std::io::{Error, ErrorKind};

pub struct RequestLine {
    pub method: Method,
    pub path: String,
    pub http_version: String
}
impl RequestLine {
    pub fn new(request_line: String) -> RequestLine {
        let mut request_arr = request_line.split(" ");

        let method_string = request_arr.next().unwrap();
        let method = Method::from_str(&method_string).unwrap();
        let path = request_arr.next().unwrap().to_string();
        let http_version = request_arr.next().unwrap().to_string();

        RequestLine { method, path, http_version }
    }
}

pub struct Request {
    pub request_line: RequestLine,
    pub content_type: Option<String>,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
    pub cache_control: Option<String>,
    pub postman_token: Option<String>,
    pub host: Option<String>,
    pub accept_encoding: Option<String>,
    pub connection: Option<String>,
    pub content_length: Option<usize>,
    pub body: Option<String>
}

impl Request {
    pub fn new(buffer: &[u8]) -> Request {
        let request_as_string = String::from_utf8_lossy(buffer);

        let mut parts = request_as_string.split("\r\n\r\n");
        let mut head = parts.next().unwrap().split("\r\n");

        let request_line = head.next().unwrap().to_string();

        let mut request = Request {
            request_line: RequestLine::new(request_line),
            content_type:None, user_agent:None, accept:None,
            cache_control:None, postman_token:None, host:None,
            accept_encoding:None, connection:None, content_length:None,
            body: None
        };

        for line in head.into_iter() {
            let mut split = line.split(": ");
            let property = split.next().unwrap();
            let value = split.next().unwrap();
            Self::set_property_by_string(&mut request, property, value.to_string());
        }

        if request.content_length == None {
            return request;
        }

        if let Some(body) = parts.next() {
            let length: usize = request.content_length.unwrap();
            request.body = Some(body.chars().take(length).collect());
        }

        request
    }

    fn set_property_by_string(&mut self, property: &str, value: String) {
        match property {
            "Content-Type" => self.content_type = Some(value),
            "User-Agent" => self.user_agent = Some(value),
            "Accept" => self.user_agent = Some(value),
            "Cache-Control" => self.cache_control = Some(value),
            "Postman-Token" => self.postman_token = Some(value),
            "Host" => self.host = Some(value),
            "Accept-Encoding" => self.accept_encoding = Some(value),
            "Connection" => self.connection = Some(value),
            "Content-Length" => self.content_length = Some(value.parse().unwrap()),
            _ => ()
        }
    }
}

pub enum Status {
    OK,
    NOTFOUND
}
impl Status {
    pub fn get_response_line(self) -> String {
        match self {
            Status::OK => "HTTP/1.1 200 OK".to_string(),
            Status::NOTFOUND => "HTTP/1.1 404 Not Found".to_string(),
        }
    }
}

#[derive(PartialEq)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE
}
impl Method {
    pub fn from_str(input: &str) -> Result<Method, Error> {
        match input {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(Error::new(ErrorKind::NotFound, "Method not found"))
        }
    }
}

pub struct Response {
    pub content: String,
    pub length: usize,
    pub status: Status
}
impl Response {
    pub fn new(content: String, status: Status) -> Response {
        let length: usize = content.len();
        Response { content, length, status }
    }
}


pub trait Save<F,Args> {
    fn save_route(routes: &mut Routes, path: &str, method: Method, func: F);
}

impl <F>Save<F,()>
    for F
where F: 'static + Fn() -> Response {
    fn save_route(routes: &mut Routes, path: &str, method: Method, func: F) {
        let route = Route::<Box<dyn Fn () -> Response>> {
            path: path.to_string(),
            method,
            function: Box::new(func)
        };
        routes.routes.push(route);
    }
}

impl <F>Save<F,Option<String>>
    for F
where F: 'static + Fn(Option<String>) -> Response {
    fn save_route(routes: &mut Routes, path: &str, method: Method, func: F) {
        let route = Route::<Box<dyn Fn (Option<String>) -> Response>> {
            path: path.to_string(),
            method,
            function: Box::new(func)
        };
        routes.body_routes.push(route);
    }
}


pub struct Routes {
    routes: Vec<Route<Box<dyn Fn() -> Response>>>,
    body_routes: Vec<Route<Box<dyn Fn(Option<String>) -> Response>>>
}

impl Routes {
    pub fn new() -> Routes {
        Routes {
            routes: Vec::new(),
            body_routes: Vec::new()
        }
    }

    pub fn get<F,Args>(mut self, path: &str, func: F) -> Routes
    where F: Save<F,Args> {
        F::save_route(&mut self, path, Method::GET, func);
        self
    }

    pub fn post<F,Args>(mut self, path: &str, func: F) -> Routes
    where F: Save<F,Args> {
        F::save_route(&mut self, path, Method::POST, func);
        self
    }

    pub fn patch<F,Args>(mut self, path: &str, func: F) -> Routes
    where F: Save<F,Args> {
        F::save_route(&mut self, path, Method::PATCH, func);
        self
    }

    pub fn delete<F,Args>(mut self, path: &str, func: F) -> Routes
    where F: Save<F,Args> {
        F::save_route(&mut self, path, Method::DELETE, func);
        self
    }

    pub fn route(self, request: Request) -> Response {
        let path = request.request_line.path;
        let method = request.request_line.method;
        for route in self.routes {
            if route.path == path && route.method == method {
                println!("found in body");
                return route.exec_func(&None);
            }
        }
        for route in self.body_routes {
            if route.path == path && route.method == method {
                return route.exec_func(&request.body);
            }
        }

        Response { content: "".to_string(), length: 0, status: Status::NOTFOUND }
    }
}


trait Exec {
    fn exec_func(self, body: &Option<String>) -> Response;
}

impl Exec for Route<Box<dyn Fn () -> Response>> {
    fn exec_func(self, _: &Option<String>) -> Response {
        (self.function)()
    }
}

impl Exec for Route<Box<dyn Fn (Option<String>) -> Response>> {
    fn exec_func(self, body: &Option<String>) -> Response {
        (self.function)(body.clone())
    }
}

pub struct Route<T> {
    path: String,
    method: Method,
    function: T
}

