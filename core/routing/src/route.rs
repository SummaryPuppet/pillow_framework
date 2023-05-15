use regex::Regex;

use pillow_http::{controller::Controller, Request, Response};

/// Route
pub struct Route {
    method: pillow_http::http_methods::HttpMethods,
    /// uri path
    uri: pillow_http::Uri,

    /// Controller Callback Function
    controller: Controller,

    /// Parameters
    params: Vec<String>,

    /// Regex
    pub(crate) regex_complete: Regex,
    pub(crate) regex_words: Regex,
}

impl Route {
    pub fn uri(&self) -> &pillow_http::Uri {
        &self.uri
    }

    pub fn method(&self) -> &pillow_http::http_methods::HttpMethods {
        &self.method
    }

    pub fn params(&self) -> &Vec<String> {
        &self.params
    }

    pub fn regex_complete(&self) -> &Regex {
        &self.regex_complete
    }

    pub fn regex_words(&self) -> &Regex {
        &self.regex_words
    }
}

impl Route {
    /// New instance of Route
    ///
    /// # Arguments
    ///
    /// * `url` - Path of Route
    /// * `controller` - Callback function
    ///
    /// # Examples
    ///
    /// ```rust
    /// Route::new("/".to_string, pillow::http::HttpMethods::GET, |request| pillow::http::Response::text("hello"))
    /// ``
    pub fn new<T>(
        url: String,
        method: pillow_http::http_methods::HttpMethods,
        controller: T,
    ) -> Self
    where
        T: Fn(&Request) -> Response + Sync + Send + 'static,
    {
        let re = Regex::new(r"(<[a-zA-Z]+>)").unwrap();
        let regex_words = Regex::new(r"([a-zA-Z0-9]+)").unwrap();

        let params = if re.is_match(&url) {
            let param_with_more = re.find(&url).unwrap().as_str().to_string();

            let r = regex_words
                .find(&param_with_more)
                .unwrap()
                .as_str()
                .to_string();

            vec![r]
        } else {
            Vec::new()
        };

        let controller = Controller::new(controller);

        Self {
            method,
            uri: pillow_http::Uri(url),
            controller,
            params,
            regex_complete: re,
            regex_words,
        }
    }
}

impl Route {
    // Parameters methods
    pub fn has_parameters(&self) -> bool {
        self.params.len() > 0
    }

    /// Add Params
    pub fn add_parameters(&mut self, param: String) {
        self.params.push(param);
    }

    /// Get params
    pub fn get_parameters(&self) -> &Vec<String> {
        &self.params
    }

    /// Use controller
    pub(crate) fn use_controller(&self, request: &Request) -> Response {
        self.controller.use_action(request)
    }
}
