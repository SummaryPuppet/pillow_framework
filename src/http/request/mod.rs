use std::collections::HashMap;

use super::http_methods::HttpMethods;

/// HTTP Request
pub struct Request {
    /// Method Like GET
    pub method: HttpMethods,
    /// Path of url
    pub path: String,
    /// Version
    pub version: u8,
    /// Params
    pub parameters: HashMap<String, String>,
}

impl Request {
    /// New instance of Request
    pub fn new(buffer: &[u8; 1024]) -> Request {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut request = httparse::Request::new(&mut headers);
        let _ = request.parse(buffer);

        let method: HttpMethods = match request.method.unwrap().to_string().as_str() {
            "GET" => HttpMethods::GET,
            "POST" => HttpMethods::POST,
            "PUT" => HttpMethods::PUT,
            "DELETE" => HttpMethods::DELETE,
            _ => HttpMethods::GET,
        };

        Request {
            method,
            path: request.path.unwrap().to_string(),
            version: request.version.unwrap(),
            parameters: HashMap::new(),
        }
    }
}

impl Request {
    /// Push param to parameters
    pub(crate) fn add_param(&mut self, param_name: String, param_value: String) {
        self.parameters.insert(param_name, param_value);
    }
}
