use crate::{Status,Response,Request,method::Method,route::{Route,Exec}};

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

