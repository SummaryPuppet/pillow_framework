use serde::{Deserialize, Serialize};

/// Htpp Methods
///
/// Like GET, POST, ...
///
/// Not supported all methods yet
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
}

impl HttpMethods {
    /// Convert HttpMethods in &str
    ///
    /// ```
    /// let method_str = HttpMethods::GET;
    /// assert_eq(method_str.as_str(), "GET")
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethods::GET => "GET",
            HttpMethods::POST => "POST",
            HttpMethods::PUT => "PUT",
            HttpMethods::DELETE => "DELETE",

            HttpMethods::HEAD => "HEAD",
            HttpMethods::OPTIONS => "OPTIONS",
            HttpMethods::CONNECT => "CONNECT",
            HttpMethods::PATCH => "PATCH",
            HttpMethods::TRACE => "TRACE",
        }
    }
}

/// Convert HttpMethods in &str
///
/// ```
/// let method_str = "GET";
/// assert_eq(from_str_to_http_method(method_str).unwrap(), HttpMethods::GET)
/// ```
pub fn from_str_to_http_method(s: &str) -> Result<HttpMethods, ()> {
    match s {
        "GET" => Ok(HttpMethods::GET),
        "POST" => Ok(HttpMethods::POST),
        "PUT" => Ok(HttpMethods::PUT),
        "DELETE" => Ok(HttpMethods::DELETE),

        "HEAD" => Ok(HttpMethods::HEAD),
        "OPTIONS" => Ok(HttpMethods::OPTIONS),
        "CONNECT" => Ok(HttpMethods::CONNECT),
        "PATCH" => Ok(HttpMethods::PATCH),
        "TRACE" => Ok(HttpMethods::TRACE),

        _ => Err(()),
    }
}
