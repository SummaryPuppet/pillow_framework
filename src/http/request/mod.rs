use std::collections::HashMap;

/// HTTP Request
pub struct Request {
    /// Method Like GET
    pub method: String,
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

        Request {
            method: request.method.unwrap().to_string(),
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
