pub mod request;
pub mod response;

use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process::Command,
};

use regex::Regex;

use request::Request;

use crate::server::ThreadPool;

pub use self::response::Response;

#[derive(Clone)]
/// Instance of Router
pub struct Router {
    addr: String,

    request: Request,

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
    /// use pillow::router::Router;
    ///
    /// let mut app = Router::new();
    /// ```
    pub fn new() -> Router {
        let mut response = Response::new();

        Router {
            addr: String::from("127.0.0.1"),

            request: Request::new(),

            _regex: Regex::new(r#"/(<[a-zA-Z]+>)/g"#).unwrap(),

            get_route: HashMap::from([
                (String::from("/resources/css/global.css"), response.css()),
                (String::from("/resources/js/main.js"), response.js()),
            ]),

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
    /// use pillow::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.get("/", |request, response| response.view("index.html"));
    /// ```
    pub fn get<F>(&mut self, uri: &str, mut controller: F)
    where
        F: FnMut(&Request, Response) -> String,
    {
        let response = Response::new();
        let request = &self.request;

        let action = controller(request, response);

        let uri = String::from(uri);

        self.get_route.insert(uri, action);
    }

    /// Method post
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.post("/", |request, response| response.view("index.html"));
    /// ```
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
    /// use pillow::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.listen("5000");
    /// ```
    pub fn listen(&self, port: &str) {
        Command::new("clear").status().unwrap();

        let port_complete = format!("{}:{}", &self.addr, &port);
        println!("Server on: http://{}", &port_complete);

        let listener = match TcpListener::bind(port_complete) {
            Ok(listener) => listener,
            Err(error) => panic!("{error}"),
        };

        let pool = ThreadPool::new(40);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let request = Request::from_stream(&mut stream);
                    let mut res: HashMap<String, String> = HashMap::new();

                    match request.method.as_str() {
                        "GET" => res = self.get_route.clone(),
                        "POST" => res = self.post_route.clone(),
                        "PUT" => {}
                        "DELETE" => {}
                        _ => {}
                    };

                    pool.execute(move || handle_connection(stream, request, &res));
                }
                Err(error) => panic!("{error}"),
            }
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
    stream.flush().unwrap();
}
