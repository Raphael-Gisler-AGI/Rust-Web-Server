use std::io::{Error, ErrorKind};

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

