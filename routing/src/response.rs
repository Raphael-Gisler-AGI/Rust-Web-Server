use crate::status::Status;

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

