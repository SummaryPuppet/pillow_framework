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
