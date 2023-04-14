/// Htpp Methods
///
/// Not supported all methods yet
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethods {
    /// Convert HttpMethods in &str
    ///
    /// ```
    ///
    /// let method_str = HttpMethods::GET;
    /// assert_eq(method_str.as_str(), "GET")
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethods::GET => "GET",
            HttpMethods::POST => "POST",
            HttpMethods::PUT => "PUT",
            HttpMethods::DELETE => "DELETE",
        }
    }
}

/// Convert &str in HttpMethods
///
/// ```
/// let method_str = "GET";
/// assert_eq(get_method_from_str(method_str), HttpMethods::GET)
/// ```
pub fn get_method_from_str(method_str: &str) -> HttpMethods {
    match method_str {
        "GET" => HttpMethods::GET,
        "POST" => HttpMethods::POST,
        "PUT" => HttpMethods::PUT,
        "DELETE" => HttpMethods::DELETE,

        _ => HttpMethods::GET,
    }
}
