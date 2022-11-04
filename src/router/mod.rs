pub mod request;
pub mod response;

use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use request::Request;

use crate::ThreadPool;

use self::response::Response;

#[derive(Clone)]
pub struct Router {
    addr: String,

    get_route: HashMap<String, String>,
    post_route: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            addr: String::from("127.0.0.1"),

            get_route: HashMap::new(),
            post_route: HashMap::new(),
        }
    }
}

impl Router {
    pub fn get<F>(&mut self, uri: &str, mut controller: F)
    where
        F: FnMut(Request, Response) -> String,
    {
        let response = Response::new();
        let request = Request::new();

        let action = controller(request, response);
        let uri = String::from(uri);

        self.get_route.insert(uri, action);
    }

    pub fn post<F>(&mut self, uri: &str, mut controller: F)
    where
        F: FnMut(Request, Response) -> String,
    {
        let response = Response::new();
        let request = Request::new();

        let action = controller(request, response);
        let uri = String::from(uri);

        self.post_route.insert(uri, action);
    }
}

impl Router {
    pub fn listen(&self, port: &str) {
        let port_complete = format!("{}:{}", &self.addr, &port);

        let listener = TcpListener::bind(port_complete).unwrap();
        let pool = ThreadPool::new(40);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let res = self.get_route.clone();

            pool.execute(move || handle_connection(stream, &res));
        }
    }
}

fn handle_connection(mut stream: TcpStream, response: &HashMap<String, String>) {
    let request = Request::from_stream(&mut stream);

    match response.get(request.path.as_str()) {
        Some(res) => send_response(&mut stream, res),
        None => {}
    }
}

fn send_response(stream: &mut TcpStream, response: &String) {
    match stream.write_all(response.as_bytes()) {
        Ok(()) => {}
        Err(error) => println!("Error: {}", error),
    };
}
