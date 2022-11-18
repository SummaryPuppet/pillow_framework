use std::collections::HashMap;

use super::response::Response;

/// Box<dyn FnMut(Request, Response) -> String>
pub type ControllerBoxType =
    Box<dyn Fn(httparse::Request, Response) -> String + Sync + Send + 'static>;

/// HashMap<String, ControllerType>
pub type ResponseHash = HashMap<String, ControllerBoxType>;

/// Routes
pub struct Routes {
    pub get: ResponseHash,
    pub post: ResponseHash,
    pub put: ResponseHash,
    pub delete: ResponseHash,
}

impl Routes {
    /// New Routes
    pub fn new() -> Routes {
        let css = make_callback(css_fn);
        let js = make_callback(js_fn);

        Routes {
            get: HashMap::from([
                ("resources/css/global.css".to_string(), css),
                ("resources/js/main.js".to_string(), js),
            ]),
            post: HashMap::new(),
            put: HashMap::new(),
            delete: HashMap::new(),
        }
    }
}

/// Create a callback from function
pub fn make_callback<'a, F>(f: F) -> ControllerBoxType
where
    F: Fn(httparse::Request, Response) -> String + Sync + Send + 'static,
{
    Box::new(f) as ControllerBoxType
}

fn css_fn(_request: httparse::Request, mut response: Response) -> String {
    response.css()
}

fn js_fn(_request: httparse::Request, mut response: Response) -> String {
    response.css()
}
