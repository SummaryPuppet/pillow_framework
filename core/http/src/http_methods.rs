#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethods {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethods::GET => "GET",
            HttpMethods::POST => "POST",
            HttpMethods::PUT => "PUT",
            HttpMethods::DELETE => "DELETE",
        }
    }
}

pub fn get_method_from_str(method_str: &str) -> HttpMethods {
    match method_str {
        "GET" => HttpMethods::GET,
        "POST" => HttpMethods::POST,
        "PUT" => HttpMethods::PUT,
        "DELETE" => HttpMethods::DELETE,

        _ => HttpMethods::GET,
    }
}
