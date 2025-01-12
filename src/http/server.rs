use super::handlers::{default_route_handler, empty_response, not_found, RouteHandler};
use super::req::{build_request, RequestMethod};
use super::res::{Response, ResponseBody};
use super::threading;

use core::panic;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

struct AppRoute {
    path: &'static str,
    method: RequestMethod,
    handler: RouteHandler,
}

pub struct HTTPServer {
    addr: String,
    routes: Vec<AppRoute>,
}

impl HTTPServer {
    fn get_route(&self, path: &str, method: &RequestMethod) -> Option<&AppRoute> {
        self.routes
            .iter()
            .find(|route| route.path == path && method.eq(&route.method))
    }

    fn route_exists(&self, path: &str, method: &RequestMethod) -> bool {
        self.get_route(path, method).is_some()
    }

    fn handle_connection(&self, mut stream: TcpStream, pool: &threading::ThreadPool) {
        let request = build_request(&stream);

        let handler = match request {
            Ok(ref request) => match self.get_route(&request.path, &request.method) {
                Some(route) => route.handler,
                None => {
                    match default_route_handler(request) {
                        Some(handler) => handler,
                        None => not_found
                    }
                },
            },
            Err(_) => empty_response,
        };

        pool.execute(move || {
            let mut response = Response::new();
            response.body = match request {
                Ok(request) => handler(request, &mut response),
                Err(err) => {
                    response.status = err.status;
                    ResponseBody::from_data(err.cause)
                }
            };

            let _ = stream.write_all(response.build_response().as_bytes());
        });
    }

    fn start_server(&self) -> ! {
        let listener = TcpListener::bind(&self.addr).unwrap();

        let pool = threading::ThreadPool::new(5);
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream, &pool);
        }
        self.start_server() // Restart the server if the loop exits for some reason
    }
}

impl HTTPServer {
    pub fn new(hostname: &str, port: u16) -> HTTPServer {
        let addr = hostname.to_string() + ":" + &port.to_string();

        HTTPServer {
            addr,
            routes: vec![],
        }
    }

    pub fn get(&mut self, path: &'static str, handler: RouteHandler) -> () {
        if self.route_exists(path, &RequestMethod::GET) {
            panic!("GET route already exists for path {}", path);
        }

        self.routes.push(AppRoute {
            path,
            method: RequestMethod::GET,
            handler,
        });
    }

    pub fn post(&mut self, path: &'static str, handler: RouteHandler) -> () {
        if self.route_exists(path, &RequestMethod::POST) {
            panic!("POST route already exists for path {}", path);
        }

        self.routes.push(AppRoute {
            path,
            method: RequestMethod::POST,
            handler,
        });
    }

    pub fn start(&self) -> ! {
        self.start_server();
    }
}
