use crate::header::{self, ContentType};
pub mod static_files;
pub mod status_code;

/// Response struct to client
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response {
    /// Like 200 OK
    status_code: StatusCode,

    /// Response Headers
    headers: HashMap<Header, String>,

    // template_engine: Template,
    /// Cross Origin Site
    pub cors: String,

    /// Content of Response
    /// Like html, json, other.
    content: Body,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Body {
    STRING(String),
    BYTES(Vec<u8>),
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Body::STRING(string) => write!(f, "{}", string),
            Body::BYTES(bytes) => {
                let bytes_str = bytes
                    .iter()
                    .map(|&byte| format!("{:02X}", byte))
                    .collect::<String>();
                write!(f, "{}", bytes_str)
            }
        }
    }
}

impl Body {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Body::STRING(s) => s.as_bytes(),

            Body::BYTES(b) => b.as_slice(),
        }
    }
}

use std::collections::HashMap;

use chrono;
use serde_json::Value;

use pillow_templates::Template;

use self::{
    header::Header,
    status_code::{AsStr, StatusCode},
};

impl Response {
    /// Returns a new Response
    pub fn new_empty() -> Response {
        Response {
            status_code: StatusCode::Successfull(status_code::Successfull::OK),

            headers: HashMap::from([
                (Header::Server, String::from("Pillow")),
                // (Header::ETag, String::from(r#""3314042""#)),
            ]),

            cors: String::from("*"),

            content: Body::STRING(String::new()),
        }
    }
}

impl Response {
    /// Send a html file from resources/views directory
    ///
    /// # Arguments
    ///
    /// * page - A String that the page on views directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::http::*;
    ///
    /// #[controller(method = "GET", path = "/")]
    /// fn index() -> Response {
    ///     Response::html("index")
    /// }
    /// ```
    pub fn html(page: &'static str) -> Response {
        let mut response = Self::new_empty();

        let html = Template::Html(page);
        let contents = html.render();

        let date = crate::get_date_now!();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::Connection, "Keep-Alive".to_string()),
            (Header::ContentLength, contents.len().to_string()),
            (Header::ContentType, "text/html; charset=utf-8".to_string()),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
        ]);

        response.insert_string_content(contents);

        response
    }

    /// Send a html file from resources/views directory
    ///
    /// # Arguments
    ///
    /// * Template - Enum contain the name of file in resources/views
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::{
    ///     http::*,
    ///     templates::{Context, Template}
    /// };
    ///
    /// #[controller(method = "GET", path = "/")]
    ///  fn index() -> Response {
    ///     let mut ctx = Context::new();
    ///
    ///     Response::view(Template::Tera("index", "tera.html", ctx))
    /// }
    /// ```
    pub fn view(template: Template) -> Response {
        let mut response = Self::new_empty();

        let contents = template.render();

        let date = crate::get_date_now!();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::Connection, "Keep-Alive".to_string()),
            (Header::ContentLength, contents.len().to_string()),
            (Header::ContentType, "text/html; charset=utf-8".to_string()),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
        ]);

        response.insert_string_content(contents);

        response
    }

    /// Send a hbs file from resources/views directory
    ///
    /// # Arguments
    ///
    /// * page - A String that the page on views directory
    /// * data - Json value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::json;
    /// use pillow::http::*;
    ///
    /// #[controller(method = "GET", path = "/")]
    ///  fn index() -> Response {
    ///     let json = json! ({
    ///         "name": "foo"
    ///     });
    ///
    ///     Response::hbs("index", json)
    /// }
    /// ```
    pub fn hbs(page: &'static str, data: Value) -> Response {
        let mut response = Self::new_empty();

        let hbs = Template::Handlebars(page, data);
        let contents = hbs.render();

        let date = crate::get_date_now!();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::Connection, "Keep-Alive".to_string()),
            (Header::ContentLength, contents.len().to_string()),
            (Header::ContentType, "text/html; charset=utf-8".to_string()),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
        ]);

        response.insert_string_content(contents);

        response
    }

    /// Send a json from macro json!
    ///
    /// # Arguments
    ///
    /// * json - A string slice that sends to http client
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::json;
    /// use pillow::http::{ MainRouter, Response };
    ///
    /// #[controller(method = "GET", path = "/")]
    ///  fn index() -> Response {
    ///     let json = json! ({
    ///         "name": "foo"
    ///     });
    ///
    ///     Response::json(json)
    /// }
    /// ```
    pub fn json(js: Value) -> Response {
        let mut response = Self::new_empty();

        let date = crate::get_date_now!();
        let json = js.to_string();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::AcceptRanges, "bytes".to_string()),
            (Header::ContentLength, json.len().to_string()),
            (
                Header::ContentType,
                "application/json; charset=utf-8".to_string(),
            ),
            (Header::Date, date.to_string()),
            (Header::Date, date.to_string()),
            (Header::Vary, "Accept-Encoding".to_string()),
        ]);

        response.insert_string_content(json);

        response
    }

    /// Send a json from str
    ///
    /// # Arguments
    ///
    /// * json - A string slice that sends to http client
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::http::*;
    ///
    /// #[controller(method = "GET", path = "/")]
    ///  fn index() -> Response {
    ///     Response::json_from_str(r#{
    ///         "name": "foo"
    ///     }#)
    /// }
    /// ```
    pub fn json_from_str(json: &str) -> Response {
        let mut response = Self::new_empty();

        let date = crate::get_date_now!();

        let json_value: Value = serde_json::from_str(json).unwrap();
        let js = json_value.to_string();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::AcceptRanges, "bytes".to_string()),
            (Header::ContentLength, js.len().to_string()),
            (
                Header::ContentType,
                "application/json; charset=utf-8".to_string(),
            ),
            (Header::Date, date.to_string()),
            (Header::Date, date.to_string()),
            (Header::Vary, "Accept-Encoding".to_string()),
        ]);

        // let response = format!("{status_line}{res}\r\n\r\n{js}");
        response.insert_string_content(js);

        response
    }

    /// Send text to client
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::http::*;
    ///
    /// #[controller(method = "GET", path = "/")]
    ///  fn index() -> Response {
    ///     Response::text("hello world")
    /// }
    /// ```
    pub fn text(txt: &str) -> Response {
        let mut response = Self::new_empty();

        let length = txt.len();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::ContentType, "text/plain".to_string()),
            (Header::ContentLength, length.to_string()),
        ]);

        response.insert_string_content(txt.to_string());

        response
    }

    /// Send css response to client
    pub fn css(&mut self, css: String) -> String {
        let status_line = self.get_status_line();

        let date = chrono::offset::Local::now();

        self.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, self.cors.to_string()),
            (Header::ContentLength, css.len().to_string()),
            // ("Content-Encoding", "br".to_string());
            (Header::ContentType, "text/css; charset=utf-8".to_string()),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
            // ("Transfer-Encoding", "chunked".to_string()),
            // ("Vary", "Accept-Encoding".to_string()),
        ]);

        let headers = self.get_headers();
        let response = format!("{status_line}{headers}\r\n\r\n{css}");

        response
    }

    /// Send javascript to client
    pub fn javascript(&mut self, js: String) -> String {
        let status_line = self.get_status_line();

        let date = chrono::offset::Local::now();

        self.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, self.cors.to_string()),
            (Header::ContentLength, js.len().to_string()),
            // ("Content-Encoding", "gzip".to_string()),
            (
                Header::ContentType,
                "application/javascript; charset=utf-8".to_string(),
            ),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
            // ("Transfer-Encoding", String::from("chunked")),
            // ("Vary", "Accept-Encoding".to_string())
        ]);

        let headers = self.get_headers();
        let response = format!("{status_line}{headers}\r\n\r\n{js}");

        response
    }

    pub fn file(content_type: ContentType, content: Vec<u8>) -> Response {
        let mut response = Self::new_empty();

        let date = crate::get_date_now!();

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::Connection, "Keep-Alive".to_string()),
            // (Header::ContentDisposition, "inline".to_string()),
            (Header::ContentLength, content.len().to_string()),
            (Header::Date, date.to_string()),
        ]);

        response.content_type(content_type);

        response.insert_bytes_content(content.clone());

        response
    }
}

impl Response {
    pub fn redirect(location: &'static str) -> Response {
        let mut response = Self::new_empty();

        response.set_status_code(StatusCode::Redirection(status_code::Redirection::Found));

        response.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, response.cors.to_string()),
            (Header::Location, location.to_string()),
        ]);

        response
    }
}

impl ToString for Response {
    /// Convert Response struct in String
    fn to_string(&self) -> String {
        let res = format!(
            "{}{}\r\n\r\n{}",
            &self.get_status_line(),
            &self.get_headers(),
            &self.content
        );

        res
    }
}

impl Response {
    /// Insert content in the Response.body
    pub fn insert_string_content(&mut self, content: String) {
        self.content = Body::STRING(content);
    }

    pub fn insert_bytes_content(&mut self, content: Vec<u8>) {
        self.content = Body::BYTES(content);
    }
}

impl Response {
    /// Change Content-Type of the response
    pub fn content_type(&mut self, content_type: header::ContentType) -> &Response {
        self.add_header(Header::ContentType, content_type.as_str().to_string());
        self
    }
}

impl Response {
    pub fn websocket_upgrade_connection() -> Response {
        let mut response = Response::new_empty();

        response.set_status_code(StatusCode::Information(
            status_code::Information::SwitchingProtocols,
        ));

        response.clear_headers();

        response.add_multiple_headers(vec![
            (Header::Upgrade, "websocket".to_string()),
            (Header::Connection, "Upgrade".to_string()),
            (
                Header::SecWebSocketAccept,
                "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=".to_string(),
            ),
            (Header::SecWebSocketProtocol, "superchat".to_string()),
        ]);

        println!("Response: {:#?}", &response.to_string());

        response
    }
}

impl Response {
    /// Add header to response
    ///
    /// # Arguments
    ///
    /// * `header` - Header name
    /// * `value` - Header value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::http::Router;
    ///
    /// let mut app = Router::new();
    ///
    /// app.get("/", |_, response| {
    ///     response.add_header(Header::ContentType, "text/hmtl".to_string());
    ///     response.view("index")
    /// })
    /// ```
    pub fn add_header(&mut self, header: Header, value: String) {
        self.headers.insert(header, value);
    }

    /// Add multiple headers to response
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::http::*;
    ///
    /// #[controller(method="GET", path = "/")]
    /// fn index() {
    ///     let response: Response = Response::new_empty();
    ///
    ///     response.add_multiple_headers(vec![
    ///         (Header::ContentType, "text/html".to_string()),
    ///         (Header::AccessControlAllowOrigin, "*".to_string())
    ///     ])
    ///
    ///     response
    /// }
    /// ```
    pub fn add_multiple_headers(&mut self, headers: Vec<(Header, String)>) {
        for (header, value) in headers {
            self.add_header(header, value);
        }
    }

    /// Get All headers in one string
    ///
    /// # Examples
    ///
    /// ```rust
    /// let headers: String = self.get_headers();
    /// ```
    pub fn get_headers(&self) -> String {
        let mut res = String::new();

        for (header, value) in &self.headers {
            let header = &header.as_str();
            res = format!("{res}\r\n{header}: {value}");
        }

        res
    }

    /// Clear All headers
    pub fn clear_headers(&mut self) {
        self.headers = HashMap::new();
    }

    /// Get Status Line
    /// HTTP/1.1 200 OK
    ///
    /// # Examples
    ///
    /// ```rust
    /// let status_line: String = self.get_status_line();
    ///
    /// assert_eq!(status_line, "HTTP/1.1 200 OK".to_string());
    /// ```
    pub fn get_status_line(&self) -> String {
        let status_code = &self.status_code;
        let status_line = format!("HTTP/1.1 {}", status_code.as_str());

        status_line
    }

    /// Set Status Code Like 200 OK
    ///
    /// # Examples
    ///
    /// ```rust
    /// let response: Response = Response::new_empty();
    /// response.set_status_code(StatusCode::Successfull(status_code::Successfull::OK));
    /// ```
    pub fn set_status_code(&mut self, code: StatusCode) {
        self.status_code = code;
    }

    pub fn get_body(&self) -> Body {
        self.content.clone()
    }
}

#[macro_export]
macro_rules! get_date_now {
    () => {{
        let date = chrono::offset::Local::now();

        date
    }};
}
