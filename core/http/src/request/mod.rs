use std::collections::HashMap;

use super::http_methods::HttpMethods;

/// HTTP Request
#[derive(Debug)]
pub struct Request {
    /// Method Like GET
    pub method: HttpMethods,
    /// Path of url
    pub path: String,
    /// Version
    pub version: u8,
    /// Params from url
    pub parameters: HashMap<String, String>,
}

impl Request {
    /// New instance of Request
    pub fn new(buffer: &[u8; 1024]) -> Request {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut request = httparse::Request::new(&mut headers);
        let _ = request.parse(buffer);

        let mut httparse_method = "";

        match request.method {
            Some(method) => httparse_method = method,
            None => println!("{:#?}", &request),
        }

        let method: HttpMethods = match httparse_method {
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
    pub fn add_param(&mut self, param_name: String, param_value: String) {
        self.parameters.insert(param_name, param_value);
    }
}
