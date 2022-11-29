use regex::Regex;

use crate::http::{controller::Controller, request::Request, response::Response};

/// Instance of Route
pub struct Route {
    /// url path
    pub url: String,
    /// Controller Callback Function
    pub controller: Controller,
    /// Parameters
    params: Vec<String>,
    /// Regex
    pub regex_complete: Regex,
    pub regex_words: Regex,
}

impl Route {
    /// New instance of Route
    ///
    /// # Arguments
    ///
    /// * `url` - Path of Route
    /// * `controller` - Callback function
    pub fn new<F>(url: String, controller: F) -> Route
    where
        F: Fn(Request, Response) -> String + Sync + Send + 'static,
    {
        let action = Controller::new(controller);
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

        Route {
            url,
            controller: action,
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

    pub fn add_parameters(&mut self, param: String) {
        self.params.push(param);
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        &self.params
    }

    // Regex methods
}
