/// Body of http
#[derive(Debug, Clone)]
pub enum Body {
    JSON(String),
    XML(String),
    HTML(String),
    NONE,
}

pub fn from_string_to_body(string: String) -> Body {
    let string = string.trim().to_string();

    if string.starts_with("<html>") {
        Body::HTML(string)
    } else if string.starts_with("{") {
        Body::JSON(string)
    } else if string.starts_with("<") {
        Body::XML(string)
    } else {
        Body::NONE
    }
}
