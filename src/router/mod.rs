pub mod request;
pub mod response;

use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use regex::Regex;

use request::Request;

use crate::server::ThreadPool;

pub use self::response::Response;

#[derive(Clone)]
pub struct Router {
    addr: String,

    _regex: Regex,

    get_route: HashMap<String, String>,
    post_route: HashMap<String, String>,
}

impl Router {
    /// Returns a new Router
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = Router::new();
    /// ```
    pub fn new() -> Router {
        Router {
            addr: String::from("127.0.0.1"),

            _regex: Regex::new(r#"/(<[a-zA-Z]+>)/g"#).unwrap(),

            get_route: HashMap::new(),
            post_route: HashMap::new(),
        }
    }
}

impl Router {
    /// Method get
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = Router::new();
    ///
    /// app.get("/", |request, response| response.view("index.html"));
    /// ```
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
    /// Method for listen in port argument
    ///
    /// # Arguments
    ///
    /// * `port` - A string slice that port
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = Router::new();
    /// app.listen("5000");
    /// ```
    pub fn listen(&self, port: &str) {
        let port_complete = format!("{}:{}", &self.addr, &port);

        let listener = TcpListener::bind(port_complete).unwrap();
        let pool = ThreadPool::new(40);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let request = Request::from_stream(&mut stream);
            let res = self.get_route.clone();

            pool.execute(move || handle_connection(stream, request, &res));
        }
    }
}

fn handle_connection(mut stream: TcpStream, request: Request, response: &HashMap<String, String>) {
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
