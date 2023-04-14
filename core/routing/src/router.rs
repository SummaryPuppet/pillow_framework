// use std::process;

// use colored::Colorize;

use std::collections::HashMap;

use super::route::Route;

// use pillow_env::Env;
use pillow_http::{handler::Handler, middlewares::Middleware, Request, Response};

/*
/// Instance of Router
pub struct Router {
    pub addr: String,

    middlewares: Vec<Middleware>,

    routes: Routes,
}

impl Router {
    /// Returns a new Router
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::Router;
    ///
    /// fn main(){
    ///     let mut app = Router::new();
    /// }
    /// ```
    pub fn new() -> Router {
        Router {
            addr: String::from("127.0.0.1"),

            middlewares: Vec::new(),

            routes: Routes::new(),
        }
    }
}

/*
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
    /// use pillow::http::Router;
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
    /// use pillow::http::Router;
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
    /// use pillow::http::Router;
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
    /// use pillow::http::Router;
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
    pub fn get_struct<T: Handler + std::fmt::Debug>(&mut self, uri: &str, controller: T) {
        println!("{}", uri);
        println!("{:#?}", controller);
    }
}

impl Router {
    /// Add a global middleware
    ///
    /// # Arguments
    ///
    /// * `controller` - Function for middleware
    pub fn add_middleware<F>(&mut self, controller: F)
    where
        F: Fn(&Request, &Response) + 'static,
    {
        self.middlewares.push(Middleware::new(controller))
    }
}

*/*/

pub struct MainRouter {
    routes: HashMap<pillow_http::http_methods::HttpMethods, Vec<Route>>,
}

impl MainRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn routes(&self) -> &HashMap<pillow_http::http_methods::HttpMethods, Vec<Route>> {
        &self.routes
    }

    fn get_routes_from_method(
        &self,
        method: &pillow_http::http_methods::HttpMethods,
    ) -> Option<&Vec<Route>> {
        self.routes.get(&method)
    }

    fn get_option_index(&self, uri: &pillow_http::Uri, routes_vec: &Vec<Route>) -> Option<usize> {
        routes_vec.iter().position(|route| route.uri() == uri)
    }

    pub fn routing(&self, request: &Request) -> Response {
        let option_routes_vec = self.get_routes_from_method(request.method());

        let routes_vec = match option_routes_vec {
            Some(routes) => routes,
            None => panic!("Routes empty"),
        };

        let option_index = self.get_option_index(request.uri(), &routes_vec);

        let mut response = Response::new_empty();

        match option_index {
            Some(index) => {
                let route_m = &routes_vec[index];

                response = route_m.use_controller(request.clone())
            }
            None => {
                let routes_params: Vec<_> = routes_vec
                    .iter()
                    .filter(|route| route.has_parameters())
                    .collect();

                for route in routes_params {
                    let path: Vec<_> = route
                        .regex_complete
                        .split(&route.uri().0.as_str())
                        .collect();

                    let path_param: Vec<_> = route
                        .regex_words
                        .find_iter(&request.uri().0.as_str())
                        .collect();

                    if request.uri().0.starts_with(path[0]) {
                        let route_m = route;

                        response = route_m.use_controller(request.clone());
                    }
                }
            }
        }

        response
    }
}

impl MainRouter {
    pub fn get<F>(&mut self, uri: &str, controller: F)
    where
        F: Fn(Request) -> Response + Sync + Send + 'static,
    {
        let uri = uri.to_string();

        self.routes
            .entry(pillow_http::http_methods::HttpMethods::GET)
            .or_insert(Vec::new())
            .push(Route::new(uri, controller));
    }
}
