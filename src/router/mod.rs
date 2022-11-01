pub mod response;

use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum HttpMethods {
    GET,
    POST,
    _PUT,
    _DELETE,
}

pub struct Router {
    get_route: HashMap<String, String>,
    post_route: HashMap<String, String>,

    request: Vec<String>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            get_route: HashMap::new(),
            post_route: HashMap::new(),

            request: Vec::new(),
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
    pub fn listen(&mut self, port: &str) {
        let ip = "127.0.0.1";
        let port_complete = format!("{}:{}", ip, &port);

        let listener = TcpListener::bind(&port_complete).unwrap();
        let pool = ThreadPool::new(30);

        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();

            //pool.execute(&mut move || handle_connection(stream, &self.get_route));

            handle_connection(stream, &self.get_route);

            /*
            self.request = get_request(&mut stream);
            send_response(&mut stream, &self.actions[0]);
            println!("{:?}", self.request)
            */
        }
    }
}

fn handle_connection(mut stream: TcpStream, response: &HashMap<String, String>) {
    let request = get_request(&mut stream);

    for res in response.values() {
        send_response(&mut stream, res)
    }

    println!("{:?}", request);
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

    request.push(route);
    request
}

fn send_response(stream: &mut TcpStream, response: &String) {
    stream.write_all(response.as_bytes()).unwrap();
}
