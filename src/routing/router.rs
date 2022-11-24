use std::process;

use colored::Colorize;
use regex::Regex;

use super::{route::Route, routes::Routes};

use crate::{
    http::{request::Request, response::Response},
    server::server_listen,
};

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
    /// use pillow::http::router::Router;
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
    /// use pillow::routing::router::Router;
    ///
    /// fn main (){
    /// let mut app = Router::new();
    ///
    /// app.get("/", |request, response| response.view("index"));
    /// }
    /// ```
    pub fn get<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);

        self.routes.get.push(Route::new(uri, controller));
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
    /// use pillow::routing::router::Router;
    ///
    /// fn main(){
    /// let mut app = Router::new();
    ///
    /// app.post("/", |request, response| {
    ///     println("{:#?}", request);
    ///
    ///     response.text("hello world")
    /// });
    /// }
    /// ```
    pub fn post<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);

        self.routes.post.push(Route::new(uri, controller));
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
    /// use pillow::routing::router::Router;
    ///
    /// fn main (){
    /// let mut app = Router::new();
    ///
    /// app.put("/", |request, response| response.text("index"));
    /// }
    /// ```
    pub fn put<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);

        self.routes.put.push(Route::new(uri, controller));
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
    /// use pillow::routing::router::Router;
    ///
    /// fn main (){
    /// let mut app = Router::new();
    ///
    /// app.delete("/", |request, mut response| response.view("index"));
    /// }
    /// ```
    pub fn delete<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(Request, Response) -> String + Sync + Send + 'static,
    {
        let uri = String::from(uri);

        self.routes.delete.push(Route::new(uri, controller));
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
    /// fn main(){
    /// let mut app = Router::new();
    ///
    /// app.listen("5000").await;
    /// }
    /// ```
    pub async fn listen(&self, port: &str) {
        // process::Command::new("clear").status().unwrap();

        let port_complete = format!("{}:{}", &self.addr, &port);
        let http = format!("http://{}", &port_complete);

        println!("Server on: [{}]", http.green());

        server_listen(port_complete, &self.routes).await;
    }
}
