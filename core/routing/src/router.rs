use std::{collections::HashMap, sync::Arc};

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

    fn response_404(&self, response: &mut Response, request: &Request) -> Response {
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
                    <h1>Not Found {} {}</h1>
                </body>
            </html>",
            &request.method().as_str(),
            &request.uri().0.as_str()
        );

        response.insert_string_content(content);

        response.clone()
    }

    fn use_controller(&self, route: &Route, request: &Request) -> Response {
        let response = route.use_controller(&request);

        response.clone()
    }

    fn search_route_with_params(
        &self,
        request: &mut Request,
        response: &mut Response,
        routes_with_params: Vec<&Route>,
    ) -> Response {
        let mut response = response.clone();

        for route in routes_with_params {
            let path: Vec<_> = route
                .regex_complete
                .split(&route.uri().0.as_str())
                .collect();

            let path_param: Vec<_> = route
                .regex_words
                .find_iter(&request.uri().0.as_str())
                .collect();

            if request.uri().0.starts_with(path[0]) {
                let key = &route.params()[0].clone();
                let value = path_param[1].as_str().to_string();

                request.add_params((key.clone(), value));

                response = self.use_controller(route, &request);
            } else {
                return self.response_404(&mut response, &request);
            }
        }

        response.clone()
    }

    pub(crate) fn routing(&self, request_ref: &Request) -> Vec<Response> {
        // Clone the request
        let mut request = request_ref.clone();

        // Create the response to send a client
        let mut response = Response::new_empty();

        let option_routes_vec = self.get_routes_from_method(request.method());
        let routes_vec: &Vec<Route>;

        match option_routes_vec {
            Some(routes) => routes_vec = routes,
            // Return 404
            None => return vec![self.response_404(&mut response, &request)],
        }

        let option_index = self.get_option_index(request.uri(), &routes_vec);

        match option_index {
            // IF find route index
            Some(index) => {
                response = self.use_controller(&routes_vec[index], &request);
            }

            //IF not find route index
            None => {
                //Find if there is any routes with params
                let routes_params: Vec<_> = routes_vec
                    .iter()
                    .filter(|route| route.has_parameters())
                    .collect();

                // IF not have params
                if routes_params.len() == 0 {
                    return vec![self.response_404(&mut response, &request)];
                }

                // Search route have the url
                response =
                    self.search_route_with_params(&mut request, &mut response, routes_params);
            }
        }

        vec![response]
    }
}

// impl<T: pillow_http::handler::Handler + Send + Sync + std::fmt::Debug> MainRouter<T> {
impl MainRouter {
    pub fn add_route_closure<T>(
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

    /// Add files from the public directory
    ///
    /// ```rust
    /// use pillow::http::*;
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///     router.public();
    /// }
    ///
    /// ```
    pub fn public(&mut self) {
        self.static_files("public");
    }

    /// Add assets from the resources directory
    ///
    /// ```rust
    /// use pillow::http::*;
    ///
    /// #[tokio::main]
    /// async fn main (){
    ///     let mut router = MainRouter::new();
    ///     router.public();
    /// }
    ///
    /// ```
    pub fn assets(&mut self) {
        self.static_files("resources/js");
        self.static_files("resources/css");
    }

    /// Insert static files in the routes
    ///
    /// ```rust
    /// fn (&mut self) {
    ///     self.static_files("static");
    /// }
    /// ```
    fn static_files(&mut self, path: &str) {
        let static_files = pillow_http::static_files::StaticFiles::new(path);

        for file in static_files.files {
            let path = file.path.clone();
            let content_type = Arc::new(file.content_type().unwrap().to_owned());

            let decoded_bytes = file.clone().content;

            let closure = move |_: &Request| -> Response {
                let content_type =
                    pillow_http::header::from_str_ext_to_content_type(&content_type.clone());

                let content = decoded_bytes.clone();

                Response::file(content_type, content.clone())
            };

            self.add_route_closure(pillow_http::http_methods::HttpMethods::GET, &path, closure);
        }
    }
}
