use std::{char, io::{BufRead, BufReader, Error, ErrorKind}, net::TcpStream};

pub struct Request {
    pub request_line: String,
    pub content_type: Option<String>,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
    pub cache_control: Option<String>,
    pub postman_token: Option<String>,
    pub host: Option<String>,
    pub accept_encoding: Option<String>,
    pub connection: Option<String>,
    pub content_length: Option<usize>,
    pub body: Vec<char>
}

//"Content-Type: application/json"
//"User-Agent: PostmanRuntime/7.37.3"
//"Accept: */*"
//"Cache-Control: no-cache"
//"Postman-Token: d6c96c69-79a7-4489-9cf4-4a0a63003390"
//"Host: localhost:7878"
//"Accept-Encoding: gzip, deflate, br"
//"Connection: keep-alive"
//"Content-Length: 49"

impl Request {
    pub fn new(buf_reader: BufReader<&mut TcpStream>) -> Request {
        let mut lines = buf_reader.lines();

        let request_line = lines.next().unwrap().unwrap();
        let mut request = Request {
            request_line, content_type:None, user_agent:None, accept:None,
            cache_control:None, postman_token:None, host:None,
            accept_encoding:None, connection:None, content_length:None,
            body: Vec::new()
        };

        loop {
            let line = lines.next().unwrap().unwrap();
            println!("{:?}", line);
            if line == "" {
                break;
            }
            let mut split = line.split(": ");
            let property = split.next().unwrap();
            let value = split.next().unwrap();
            Self::set_property_by_string(&mut request, property, value.to_string());
        }

        if request.content_length == None {
            return request;
        }

        let length: usize = request.content_length.unwrap();

        let mut counter = 0;

        while counter < length {
            // " at the start and end of the line
            counter += 2;
            let line = lines.next().unwrap().unwrap();
            for c in line.chars() {
                counter += 1;
                request.body.push(c);
            }
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

