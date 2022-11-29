use crate::http::{request::Request, response::Response};

use super::route::Route;

/// Routes
pub struct Routes {
    pub get: Vec<Route>,
    pub post: Vec<Route>,
    pub put: Vec<Route>,
    pub delete: Vec<Route>,
}

impl Routes {
    /// New Routes
    pub fn new() -> Routes {
        Routes {
            get: vec![
                Route::new("resources/css/global.css".to_string(), css_fn),
                Route::new("resources/js/main.js".to_string(), js_fn),
            ],
            post: Vec::new(),
            put: Vec::new(),
            delete: Vec::new(),
        }
    }
}

fn css_fn(_request: Request, mut response: Response) -> String {
    response.css()
}

fn js_fn(_request: Request, mut response: Response) -> String {
    response.javascript()
}
