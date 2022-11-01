pub mod response;

use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::ThreadPool;

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
    pub fn get(&mut self, uri: String, action: String) {
        self.get_route.insert(uri, action);
    }

    pub fn post(&mut self, uri: String, action: String) {
        self.post_route.insert(uri, action);
    }
}

impl Router {
    pub fn listen(&self, port: &str) {
        let port_complete = format!("{}:{}", &self.addr, &port);

        let listener = TcpListener::bind(port_complete).unwrap();
        let pool = ThreadPool::new(70);

        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();
            let res = self.get_route.clone();

            pool.execute(move || handle_connection(stream, &res));
        }
    }
}

fn handle_connection(mut stream: TcpStream, response: &HashMap<String, String>) {
    let request = get_request(&mut stream);

    match response.get(request[0].as_str()) {
        Some(res) => send_response(&mut stream, res),
        None => {}
    }
}

fn get_request(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    return_route(http_request)
}

fn return_route(mut request: Vec<String>) -> Vec<String> {
    let mut route = String::new();

    for s in request[0].split_whitespace() {
        if s.starts_with("/") {
            route = String::from(s);
            break;
        }
    }

    request.insert(0, route);
    request
}

fn send_response(stream: &mut TcpStream, response: &String) {
    stream.write_all(response.as_bytes()).unwrap();
}
