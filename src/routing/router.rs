use std::process;

use colored::Colorize;

use super::{route::Route, routes::Routes};

use crate::{
    env::Env,
    http::{request::Request, response::Response},
    server::server_listen,
};

/// Instance of Router
pub struct Router {
    addr: String,

    routes: Routes,
}

impl Router {
    /// Returns a new Router
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::routing::router::Router;
    ///
    /// fn main(){
    ///     let mut app = Router::new();
    /// }
    /// ```
    pub fn new() -> Router {
        Router {
            addr: String::from("127.0.0.1"),

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
    ///     let mut app = Router::new();
    ///
    ///     app.get("/", |_, mut response| response.view("index"));
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
    /// #[async_std::main]
    /// async fn main(){
    /// let mut app = Router::new();
    ///
    /// app.listen("5000").await;
    /// }
    /// ```
    pub async fn listen(&self, port: &str) {
        if !Env::is_var_exist("APP_DEBUG".to_string()) {
            process::Command::new("clear").status().unwrap();
        }

        let port_complete = format!("{}:{}", &self.addr, &port);
        let http = format!("http://{}", &port_complete);

        println!("Pillow on: [{}]", http.green());

        server_listen(port_complete, &self.routes).await;
    }
}

#[macro_export]
macro_rules! pillow_create_server {
    () => {{
        let app = Router::new();

        app
    }};
}
