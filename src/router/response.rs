use std::fs;

pub struct Response {
    status_line: String,
}

use chrono;
use serde_json::Value;

impl Response {
    pub fn new() -> Response {
        Response {
            status_line: String::from("HTTP/1.1 200 OK"),
        }
    }
}

impl Response {
    pub fn view(&self, page: String) -> String {
        let status_line = &self.status_line;

        let path = format!("views/{page}");
        let contents = fs::read_to_string(&path).unwrap();
        let length = contents.len();
        let e_tag = r#""3314042""#;
        let date = chrono::offset::Local::now();

        let response = format!("{status_line}\r\nDate: {date}\r\nServer: Sunny\r\nLast-Modified: {date}\r\nETag: {e_tag}\r\nAccept-Ranges: bytes\r\nContent-Length: {length}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{contents}", );

        response
    }

    pub fn json(&self, js: &str) -> String {
        let status_line = &self.status_line;
        let length = js.trim().len();

        let e_tag = r#""3314042""#;
        let date = chrono::offset::Local::now();

        let json: Value = serde_json::from_str(&js).unwrap();

        let response = format!("{status_line}\r\nDate: {date}\r\nServer: Sunny\r\nLast-Modified: {date}\r\nETag: {e_tag}\r\nAccept-Ranges: bytes\r\nVary: Accept-Encoding\r\nContent-Length: {length}\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{json}", );
        println!("{}", &response);

        response
    }

    pub fn text(&self, txt: String) -> String {
        let status_line = &self.status_line;
        let length = txt.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{txt}");

        response
    }
}
