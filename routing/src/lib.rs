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
    UPDATE,
    DELETE
}
impl Method {
    pub fn from_str(input: &str) -> Result<Method, Error> {
        match input {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "UPDATE" => Ok(Method::UPDATE),
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

pub struct Routes {
    routes: Vec<Route>
}

impl Routes {
    pub fn new() -> Routes {
        Routes { routes: vec![] }
    }

    pub fn add(mut self, path: &str, method: Method, function: fn() -> Response) -> Routes {
        let route: Route = Route::new(path, method, function);
        self.routes.push(route);
        return self;
    }

    pub fn route(self, path: &str, method: Method) -> Response {
        for route in self.routes {
            if route.path == path && route.method == method {
                return (route.function)();
            }
        }

        Response { content: "".to_string(), length: 0, status: Status::NOTFOUND }
    }
}

pub struct Route {
    path: String,
    method: Method,
    function: fn() -> Response,
}

impl Route {
    pub fn new(path: &str, method: Method, function: fn() -> Response) -> Route {
        Route { path: path.to_string(), method, function }
    }
}

