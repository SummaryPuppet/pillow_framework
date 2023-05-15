use std::collections::HashMap;
use std::fmt;

use crate::header::Header;
use crate::http_methods::HttpMethods;
use crate::uri::Uri;

use crate::body::Body;

/// Http Request
#[derive(Debug, Clone)]
pub struct Request {
    method: HttpMethods,
    version: String,
    headers: HashMap<Header, String>,
    uri: Uri,
    params: HashMap<String, String>,
    body: Body,
}

impl Default for Request {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl Request {
    /// Instance empty Request
    pub fn new_empty() -> Self {
        Self {
            method: HttpMethods::GET,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            uri: Uri("".to_string()),
            params: HashMap::new(),
            body: Body::NONE,
        }
    }
}

impl Request {
    /// http method reference
    pub fn method(&self) -> &HttpMethods {
        &self.method
    }

    /// version reference
    pub fn version(&self) -> &String {
        &self.version
    }

    /// uri reference
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Headers reference
    pub fn headers(&self) -> &HashMap<Header, String> {
        &self.headers
    }

    /// Params reference
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Body reference
    pub fn body(&self) -> &Body {
        &self.body
    }
}

impl Request {
    /// Create a Request from &Vec<u8>
    ///
    /// # Arguments
    ///
    /// * data - Is a data from a stream
    pub fn from_vec(data: &Vec<u8>) -> Result<Request, std::str::Utf8Error> {
        let request_str_full = Self::from_vec_to_str(data)?;

        let mut req_vec: Vec<&str> = request_str_full.split("\n").collect();

        let (method_str, uri_str, version_str) = Self::separate_method_uri_version(req_vec[0]);

        let method = crate::http_methods::from_str_to_http_method(method_str).unwrap();
        let uri = Self::get_uri(uri_str);

        let params = match Self::get_params(uri_str) {
            Some(hashmap) => hashmap,
            None => HashMap::new(),
        };

        let headers = Self::get_headers(&mut req_vec);
        let body = Self::get_body(req_vec, headers.len());

        Ok(Self {
            method,
            version: version_str.to_string(),
            uri,
            headers,
            params,
            body,
        })
    }

    /// Create a body from a Vec<&str>
    ///
    /// # Arguments
    ///
    /// * header_vec - All header in a request
    ///
    /// * lenght - body lenght
    fn get_body(headers_vec: Vec<&str>, lenght: usize) -> crate::body::Body {
        let mut body_vec: Vec<Vec<&str>> = headers_vec.chunks(lenght).map(|x| x.into()).collect();
        body_vec.remove(0);

        let body_vec = body_vec[0].clone();

        let mut body = String::new();

        for body_items in &body_vec {
            if !body_vec.is_empty() {
                body = body + body_items;
            }
        }

        let body = Self::remove_0(&body);

        crate::body::from_string_to_body(body.to_string())
    }

    /// Remove a 0 in last part from request
    ///
    /// # Arguments
    ///
    /// * string - String where delete 0
    fn remove_0(string: &String) -> &str {
        let vec_str: Vec<&str> = string.split("\0").collect();

        vec_str[0]
    }

    /// Get a headers
    fn get_headers(headers_vec: &mut Vec<&str>) -> HashMap<Header, String> {
        headers_vec.remove(0);

        let mut header_hash_map = HashMap::new();

        for header in headers_vec {
            let key_value_vec: Vec<&str> = header.split(":").map(|x| x.trim()).collect();

            if key_value_vec.len() > 1 {
                let key = crate::header::from_string_to_header(key_value_vec[0].to_string());
                let value = key_value_vec[1].to_string();

                if key_value_vec.len() > 2 {
                    let value = value + key_value_vec[2];
                    header_hash_map.insert(key, value);
                } else {
                    header_hash_map.insert(key, value);
                }
            }
        }

        header_hash_map
    }

    /// Separate method, uri, version from a all string request
    fn separate_method_uri_version(header: &str) -> (&str, &str, &str) {
        let header_vec: Vec<&str> = header.split_whitespace().collect();

        (header_vec[0], header_vec[1], header_vec[2])
    }

    /// Convert data to Result of &str is a request
    fn from_vec_to_str(data: &Vec<u8>) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(data)
    }

    /// Convert &str to Uri struct
    fn get_uri(uri_str: &str) -> Uri {
        let uri_vec: Vec<&str> = uri_str.split("?").collect();

        Uri(uri_vec[0].to_string())
    }

    /// Get params in the uri
    fn get_params(uri: &str) -> Option<HashMap<String, String>> {
        let mut params_vec: Vec<&str> = uri.split("?").collect();
        let mut params_hash_map = HashMap::new();

        if params_vec.len() > 1 {
            params_vec.remove(0);

            let params_vec: Vec<&str> = params_vec[0].split("&").collect();

            for params in params_vec {
                let p_vec: Vec<&str> = params.split("=").collect();

                params_hash_map.insert(p_vec[0].to_string(), p_vec[1].to_string());
            }

            return Some(params_hash_map);
        }

        None
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .field("uri", &self.uri)
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
    }
}
