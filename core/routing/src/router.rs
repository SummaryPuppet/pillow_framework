use std::collections::HashMap;

use crate::route::Route;

#[allow(unused_imports)]
use pillow_http::{handler::Handler, middlewares::Middleware, Request, Response};

/// The Main router in your app
///
/// ```rust
/// #[tokio::main]
/// async fn main(){
///     let mut router = MainRouter::new();
/// }
/// ```
#[allow(dead_code)]
pub struct MainRouter {
    routes: HashMap<pillow_http::http_methods::HttpMethods, Vec<Route>>,
}

impl MainRouter {
    /// Instance of a router
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Reference of routes
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

    pub(crate) fn routing(&self, request: &Request) -> Response {
        let option_routes_vec = self.get_routes_from_method(request.method());

        let routes_vec = match option_routes_vec {
            Some(routes) => routes,
            None => panic!("Routes empty"),
        };

        let option_index = self.get_option_index(request.uri(), &routes_vec);
        let mut response = Response::new_empty();

        match option_index {
            // IF find route index
            Some(index) => {
                let route_m = &routes_vec[index];
                response = route_m.use_controller(request)
            }
            //IF not find route index
            None => {
                //Find if route has params
                let routes_params: Vec<_> = routes_vec
                    .iter()
                    .filter(|route| route.has_parameters())
                    .collect();

                // IF not have params
                if routes_params.len() == 0 {
                    response.set_status_code(pillow_http::status_code::StatusCode::ClientError(
                        pillow_http::status_code::ClientError::NotFound,
                    ));

                    response.add_header(
                        pillow_http::header::Header::ContentType,
                        "text/html".to_string(),
                    );

                    let content = format!(
                        "<html>
                    <head>
                        <style>
                            * {{
                                margin: 0;
                                padding: 0;
                            }}
                            body {{
                                height: 100vh;
                                display: flex;
                                justify-content: center;
                                align-items: center;
                                background-color: #080808;
                            }}
                            h1 {{
                                font-family: system-ui;
                                color: #fefefe;
                            }}
                        </style>
                    </head>
                    <body>
                        <h1>Not Found {}</h1>
                    </body>
                    </html>",
                        &request.uri().0.as_str()
                    );

                    response.insert_content(content);
                }

                // Search route have the url
                for route in routes_params {
                    let path: Vec<_> = route
                        .regex_complete
                        .split(&route.uri().0.as_str())
                        .collect();

                    let path_param: Vec<_> = route
                        .regex_words
                        .find_iter(&request.uri().0.as_str())
                        .collect();

                    println!("{:#?}", path_param);

                    if request.uri().0.starts_with(path[0]) {
                        let route_m = route;
                        response = route_m.use_controller(request);
                    }
                }
            }
        }

        response
    }
}

// impl<T: pillow_http::handler::Handler + Send + Sync + std::fmt::Debug> MainRouter<T> {
impl MainRouter {
    pub fn add_route_clousure<T>(
        &mut self,
        method: pillow_http::http_methods::HttpMethods,
        path: &str,
        controller: T,
    ) where
        T: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        let uri = path.to_string();

        self.routes
            .entry(method)
            .or_insert(Vec::new())
            .push(Route::new(uri, method, controller));
    }

    /// Add Route
    ///
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::*;
    ///
    /// #[controller(method = "GET", path = "/")]
    /// fn index (_: Request) -> Response {
    ///     Response::text("hello")
    /// }
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///
    ///     router.add_route(route!(index {}));
    /// }
    /// ```
    pub fn add_route(&mut self, route: Route) {
        self.routes
            .entry(route.method().clone())
            .or_insert(Vec::new())
            .push(route)
    }
}

impl MainRouter {
    /// Method GET
    ///
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::{MainRouter, Response};
    ///
    /// fn index (_: Request) -> Response {
    ///     Response::text("hello")
    /// }
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///
    ///     router.get("/", index);
    /// }
    /// ```
    pub fn get<T>(&mut self, uri: &str, controller: T)
    where
        T: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.add_route_clousure(pillow_http::http_methods::HttpMethods::GET, uri, controller)
    }

    /// Method POST
    ///
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::*;
    ///
    /// fn index (_: Request) -> Response {
    ///     Response::text("hello")
    /// }
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///
    ///     router.post("/", index);
    /// }
    /// ```
    pub fn post<T>(&mut self, uri: &str, controller: T)
    where
        T: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.add_route_clousure(
            pillow_http::http_methods::HttpMethods::POST,
            uri,
            controller,
        )
    }

    /// Method PUT
    ///
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::*;
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///
    ///     router.put("/", |_, | Response::view("index"));
    /// }
    /// ```
    pub fn put<T>(&mut self, uri: &str, controller: T)
    where
        T: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.add_route_clousure(pillow_http::http_methods::HttpMethods::PUT, uri, controller)
    }

    /// Method DELETE
    /// # Arguments
    ///
    /// * `uri` - Path of route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::http::{MainRouter, Response};
    ///
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///
    ///     router.delete("/", |_, | Response::view("index"));
    /// }
    /// ```
    pub fn delete<T>(&mut self, uri: &str, controller: T)
    where
        T: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.add_route_clousure(
            pillow_http::http_methods::HttpMethods::DELETE,
            uri,
            controller,
        )
    }
}
