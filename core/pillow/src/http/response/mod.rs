mod header;
pub mod static_files;
mod status_code;
mod view;

/// Response to client
pub struct Response {
    /// Like 200 OK
    status_code: StatusCode,

    /// Response Headers
    headers: HashMap<Header, String>,

    /// Cross Origin Site
    pub cors: String,
}

use std::collections::HashMap;

use chrono;
use serde_json::Value;

use self::{
    header::Header,
    status_code::{AsStr, StatusCode},
    view::View,
};

impl Response {
    /// Returns a new Response
    pub fn new() -> Response {
        Response {
            status_code: StatusCode::Successfull(status_code::Successfull::OK),

            headers: HashMap::from([
                (Header::Server, String::from("Pillow")),
                (Header::Date, String::new()),
                (Header::ETag, String::from(r#""3314042""#)),
                (Header::LastModified, String::new()),
                (Header::AccessControlAllowOrigin, String::new()),
                (Header::CacheControl, String::from("public, max-age=0")),
                (Header::ContentType, String::new()),
                (
                    Header::ContentSegurityPolicy,
                    String::from("default-src https:"),
                ),
            ]),

            cors: String::from("*"),
        }
    }
}

impl Response {
    /// Send a html file from views directory
    ///
    /// # Arguments
    ///
    /// * page - A String that the page on views directory
    ///
    /// # Examples
    ///
    /// ```
    /// app.get("/", |_, mut response| response.view("index"));
    /// ```
    pub fn view(&mut self, page: &str) -> String {
        let status_line = &self.get_status_line();

        let view = View::new();
        let contents = view.render_page(page.to_string());

        let date = crate::get_date_now!();

        self.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, self.cors.to_string()),
            (Header::Connection, "Keep-Alive".to_string()),
            (Header::ContentLength, contents.len().to_string()),
            (Header::ContentType, "text/html; charset=utf-8".to_string()),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
        ]);

        let headers = self.get_headers();
        let response = format!("{status_line}{headers}\r\n\r\n{contents}");

        response
    }

    /// Send a hbs file from views directory
    ///
    /// # Arguments
    ///
    /// * page - A String that the page on views directory
    /// * data - Json value
    ///
    /// # Examples
    ///
    /// ```
    /// use pillow::json;
    ///
    /// app.get("/", |_, mut response| {
    ///     response.view_hbs("index", json!({"name": "foo"}))
    /// });
    /// ```
    pub fn view_hbs(&mut self, page: &str, data: Value) -> String {
        let status_line = &self.get_status_line();

        let view = View::new();
        let contents = view.render_handlebars(page.to_string(), data);

        let date = crate::get_date_now!();

        self.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, self.cors.to_string()),
            (Header::Connection, "Keep-Alive".to_string()),
            (Header::ContentLength, contents.len().to_string()),
            (Header::ContentType, "text/html; charset=utf-8".to_string()),
            (Header::Date, date.to_string()),
            (Header::LastModified, date.to_string()),
        ]);

        let headers = self.get_headers();
        let response = format!("{status_line}{headers}\r\n\r\n{contents}");

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
    /// ```
    /// use pillow::json;
    ///
    /// fn (){
    /// app.get("/", |_, mut response| {
    ///    let json = json!({
    ///     "name": "Manuel"
    ///     "age": 18
    ///    })
    ///
    ///    response.json(json)
    /// }};
    /// ```
    pub fn json(&mut self, js: Value) -> String {
        let status_line = &self.get_status_line();
        let date = crate::get_date_now!();
        let json = js.to_string();

        self.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, self.cors.to_string()),
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

        let res = self.get_headers();
        let response = format!("{status_line}{res}\r\n\r\n{js}");

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
    /// ```
    /// fn (){
    /// app.get("/", |_, mut response| response.json_from_str(r#"
    /// {
    ///     "name": "SummaryPuppet",
    ///     "age": 18,
    /// }
    /// "#));
    /// }
    /// ```
    pub fn json_from_str(&mut self, json: &str) -> String {
        let status_line = &self.get_status_line();
        let date = crate::get_date_now!();

        let json_value: Value = serde_json::from_str(json).unwrap();
        let js = json_value.to_string();

        self.add_multiple_headers(vec![
            (Header::AccessControlAllowOrigin, self.cors.to_string()),
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

        let res = self.get_headers();
        let response = format!("{status_line}{res}\r\n\r\n{js}");

        response
    }

    /// Send text to client
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn (){
    ///     app.get("/", |_, response| response.text("hello"));
    /// }
    /// ```
    pub fn text(&self, txt: &str) -> String {
        let status_line = &self.get_status_line();
        let length = txt.len();

        let response = format!("{status_line}\r\nAccess-Control-Allow-Origin: {}\r\nContent-Length: {length}\r\n\r\n{txt}", self.cors);

        response
    }

    /// Send css response to client
    pub(crate) fn css(&mut self, css: String) -> String {
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
    pub(crate) fn javascript(&mut self, js: String) -> String {
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
}

impl Response {
    pub(crate) fn websocket_upgrade_connection(&mut self) -> String {
        self.set_status_code(StatusCode::Information(
            status_code::Information::SwitchingProtocols,
        ));

        let status_line = self.get_status_line();

        self.clear_headers();

        self.add_multiple_headers(vec![
            (Header::Upgrade, "websocket".to_string()),
            (Header::Connection, "Upgrade".to_string()),
            (Header::SecWebSocketAccept, "fwaeawgeegaw".to_string()),
            (Header::SecWebSocketProtocol, "superchat".to_string()),
        ]);

        let headers = self.get_headers();

        let response = format!("{}{}\r\n", status_line, headers);

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
    /// app.get("/", |_, response|{
    ///     response.add_multiple_headers(vec![
    ///         (Header::ContentType, "text/html".to_string()),
    ///         (Header::AccessControlAllowOrigin, "*".to_string())
    ///     ])
    ///
    ///     response.view("index")
    /// })
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
    fn get_headers(&self) -> String {
        let mut res = String::new();

        for (header, value) in &self.headers {
            let header = &header.as_str();
            res = format!("{res}\r\n{header}: {value}");
        }

        res
    }

    /// Clear All headers
    fn clear_headers(&mut self) {
        self.headers = HashMap::new();
    }

    /// Get Status Line
    fn get_status_line(&self) -> String {
        let status_code = &self.status_code;
        let status_line = format!("HTTP/1.1 {}", status_code.as_str());

        status_line
    }

    /// Set Status Code Like 200 OK
    pub fn set_status_code(&mut self, code: StatusCode) {
        self.status_code = code;
    }
}

#[macro_export]
macro_rules! get_date_now {
    () => {{
        let date = chrono::offset::Local::now();

        date
    }};
}
