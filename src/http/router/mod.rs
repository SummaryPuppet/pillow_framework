use std::process;

use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpListener,
};

use futures::StreamExt;
use regex::Regex;

use super::routes::{make_callback, Routes};

use crate::http::response::Response;

/// Instance of Router
pub struct Router {
    addr: String,

    _regex: Regex,

    routes: Routes,
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
        Router {
            addr: String::from("127.0.0.1"),

            _regex: Regex::new(r"(<[a-zA-Z]+>)").unwrap(),

            routes: Routes::new(),
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
    /// use pillow::http::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.get("/", |request, response| response.view("index"));
    /// ```
    pub fn get<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(httparse::Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);
        let callback = make_callback(controller);

        self.routes.get.insert(uri, callback);
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
    /// use pillow::http::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.post("/", |request, response| response.view("index"));
    /// ```
    pub fn post<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(httparse::Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);
        let callback = make_callback(controller);

        self.routes.post.insert(uri, callback);
    }

    /// Method put
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.put("/", |request, response| response.view("index"));
    /// ```
    pub fn put<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(httparse::Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);
        let callback = make_callback(controller);

        self.routes.put.insert(uri, callback);
    }

    /// Method delete
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.delete("/", |request, response| response.view("index"));
    /// ```
    pub fn delete<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(httparse::Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);
        let callback = make_callback(controller);

        self.routes.delete.insert(uri, callback);
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
    /// use pillow::http::router::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.listen("5000");
    /// ```
    pub async fn listen(&self, port: &str) {
        process::Command::new("clear").status().unwrap();

        let port_complete = format!("{}:{}", &self.addr, &port);
        println!("Server on: http://{}", &port_complete);

        let listener = TcpListener::bind(port_complete).await.unwrap();

        listener
            .incoming()
            .for_each_concurrent(None, |tcpstream| async move {
                let mut stream = tcpstream.unwrap();

                let mut buffer = [0; 1024];
                let mut headers = [httparse::EMPTY_HEADER; 16];

                stream.read(&mut buffer).await.unwrap();

                let mut request = httparse::Request::new(&mut headers);
                let _res = request.parse(&buffer);

                let res = match request.method.unwrap() {
                    "GET" => &self.routes.get,
                    "POST" => &self.routes.post,
                    "PUT" => &self.routes.put,
                    "DELETE" => &self.routes.delete,
                    _ => &self.routes.get,
                };

                match res.get(request.path.unwrap()) {
                    Some(res) => {
                        let r = res(request, Response::new());
                        stream.write_all(r.as_bytes()).await.unwrap();
                    }
                    None => {
                        println!("Aqui mori")
                    }
                }
            })
            .await;
    }
}
