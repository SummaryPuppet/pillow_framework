use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    pub headers: Vec<String>,
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            headers: Vec::new(),
            method: String::new(),
            path: String::new(),
        }
    }

    pub fn from_stream(stream: &mut TcpStream) -> Request {
        let buf_reader = BufReader::new(stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let status_line = http_request[0].split_whitespace();
        let mut method = String::new();
        let mut path = String::new();

        for s in status_line {
            match s {
                "GET" => method = s.to_string(),
                "POST" => method = s.to_string(),
                "PUT" => method = s.to_string(),
                "DELETE" => method = s.to_string(),
                other => {
                    if other.starts_with("/") {
                        path = other.to_string();
                    }
                }
            }
        }

        Request {
            method,
            path,
            headers: http_request,
        }
    }
}
