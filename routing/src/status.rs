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
