use crate::{Response,method::Method};

pub trait Exec {
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
    pub path: String,
    pub method: Method,
    pub function: T
}

