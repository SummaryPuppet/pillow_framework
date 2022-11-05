mod view;

pub struct Response {
    /// Status Line
    /// Like HTTP/1.1 200 OK
    status_line: String,
    /// Response Headers
    headers: HashMap<String, String>,
    /// Cross Origin Site
    pub cors: String,
}

use std::collections::HashMap;

use chrono;

use self::view::View;

impl Response {
    /// Returns a new Response
    pub fn new() -> Response {
        Response {
            status_line: String::from("HTTP/1.1 200 OK"),
            headers: HashMap::from([
                (String::from("Server"), String::from("Pillow")),
                (String::from("Date"), String::new()),
                (String::from("ETag"), String::from(r#""3314042""#)),
                (String::from("Last-Modified"), String::new()),
                (String::from("Access-Control-Allow-Origin"), String::new()),
                (String::from("Content-Type"), String::new()),
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
    /// app.get("/", |_, response| response.view("index.html"));
    /// ```
    pub fn view(&mut self, page: String) -> String {
        let status_line = String::from("HTTP/1.1 200 OK");

        let view = View::new();
        let contents = view.render_page(page);

        let date = chrono::offset::Local::now();

        self.add_header("Access-Control-Allow-Origin", self.cors.to_string());
        self.add_header("Connection", "Keep-Alive".to_string());
        self.add_header("Content-Length", contents.len().to_string());
        self.add_header("Content-Type", "text/html; charset=utf-8".to_string());
        self.add_header("Date", date.to_string());
        self.add_header("Last-Modified", date.to_string());

        let mut res = String::new();

        for (header, value) in &self.headers {
            res = format!("{res}\r\n{header}: {value}");
        }

        let response = format!("{status_line}{res}\r\n\r\n{contents}");

        response
    }

    /// Send a json
    ///
    /// # Arguments
    ///
    /// * json - A string slice that sends to http client
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    ///
    /// let json = json!({
    ///     "name": "SummaryPuppet",
    ///     "age": 18
    /// })
    ///
    /// app.get("/", |_, response| response.json(js));
    /// ```
    pub fn json(&mut self, js: String) -> String {
        let status_line = String::from("HTTP/1.1 200 OK");
        let date = chrono::offset::Local::now();

        self.add_header("Access-Control-Allow-Origin", self.cors.to_string());
        self.add_header("Accept-Ranges", "bytes".to_string());
        self.add_header("Content-Length", js.len().to_string());
        self.add_header(
            "Content-Type",
            "application/json; charset=utf-8".to_string(),
        );
        self.add_header("Date", date.to_string());
        self.add_header("Last-Modified", date.to_string());
        self.add_header("Vary", "Accept-Encoding".to_string());

        let mut res = String::new();

        for (header, value) in &self.headers {
            res = format!("{res}\r\n{header}: {value}");
        }

        let response = format!("{status_line}{res}\r\n\r\n{js}");

        response
    }

    pub fn text(&self, txt: String) -> String {
        let status_line = &self.status_line;
        let length = txt.len();

        let response = format!("{status_line}\r\nAccess-Control-Allow-Origin: {}\r\nContent-Length: {length}\r\n\r\n{txt}", self.cors);

        response
    }

    pub fn css(&mut self) -> String {
        let status_line = String::from("HTTP/1.1 200 OK");

        let view = View::new();
        let (css, _) = view.static_files();

        let date = chrono::offset::Local::now();

        self.add_header("Access-Control-Allow-Origin", self.cors.to_string());
        self.add_header("Content-Length", css.len().to_string());
        self.add_header("Content-Encoding", "gzip".to_string());
        self.add_header("Content-Type", "text/css; charset=utf-8".to_string());
        self.add_header("Date", date.to_string());
        self.add_header("Last-Modified", date.to_string());
        self.add_header("Transfer-Encoding", String::from("chunked"));
        self.add_header("Vary", "Accept-Encoding".to_string());

        let mut res = String::new();

        for (header, value) in &self.headers {
            res = format!("{res}\r\n{header}: {value}");
        }

        let response = format!("{status_line}{res}\r\n\r\n{css}");

        response
    }

    pub fn js(&mut self) -> String {
        let status_line = String::from("HTTP/1.1 200 OK");

        let view = View::new();
        let (_, js) = view.static_files();

        let date = chrono::offset::Local::now();

        self.add_header("Access-Control-Allow-Origin", self.cors.to_string());
        self.add_header("Content-Length", js.len().to_string());
        self.add_header("Content-Encoding", "gzip".to_string());
        self.add_header(
            "Content-Type",
            "application/javascript; charset=utf-8".to_string(),
        );
        self.add_header("Date", date.to_string());
        self.add_header("Last-Modified", date.to_string());
        self.add_header("Transfer-Encoding", String::from("chunked"));
        self.add_header("Vary", "Accept-Encoding".to_string());

        let mut res = String::new();

        for (header, value) in &self.headers {
            res = format!("{res}\r\n{header}: {value}");
        }

        let response = format!("{status_line}{res}\r\n\r\n{js}");

        response
    }
}

impl Response {
    pub fn add_header(&mut self, header: &str, value: String) {
        self.headers.insert(header.to_string(), value);
    }
}
