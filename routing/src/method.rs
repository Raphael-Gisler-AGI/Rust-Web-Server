use std::io::{Error, ErrorKind};

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
