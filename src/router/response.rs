use std::fs;

pub struct Response {}

impl Response {
    pub fn view(page: String) -> String {
        let status_line = "HTTP/1.1 200 OK";

        let path = format!("views/{page}");
        let contents = fs::read_to_string(path).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        response
    }

    pub fn json(js: String) -> String {
        let status_line = "HTTP/1.1 200 OK";
        let length = js.len();

        let response = format!("{status_line}\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{js}");

        response
    }

    pub fn text(txt: String) -> String {
        let status_line = "HTTP/1.1 200 OK";
        let length = txt.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{txt}\r\nContent-Type: text/plain"
        );

        response
    }
}
