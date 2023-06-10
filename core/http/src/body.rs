use serde_json::Value;

/// Body of http
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    /// Json format
    JSON(Value),

    /// XML format
    XML(String),

    /// HTML format
    HTML(String),

    /// For other formats
    NONE,
}

/// Convert a string in Body enum
pub fn from_string_to_body(string: String) -> Body {
    let string = string.trim().to_string();

    if string.starts_with("<html>") {
        Body::HTML(string)
    } else if string.starts_with("{") {
        let json = serde_json::from_str(string.as_str()).unwrap();

        Body::JSON(json)
    } else if string.starts_with("<") {
        Body::XML(string)
    } else {
        Body::NONE
    }
}
