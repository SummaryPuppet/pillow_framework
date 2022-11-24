use regex::Regex;

use crate::http::{request::Request, response::Response};

type ControllerBoxType = Box<dyn Fn(Request, Response) -> String + Sync + Send + 'static>;

/// Instance of Route
pub struct Route {
    /// url path
    pub url: String,
    /// Controller Callback Function
    pub action: ControllerBoxType,
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
        let action = Box::new(controller);
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
            action,
            params,
            regex_complete: re,
            regex_words,
        }
    }
}

impl Route {
    // Action methods
    pub fn get_action(&self) -> &ControllerBoxType {
        &self.action
    }

    pub fn use_action(&self, request: Request, response: Response) -> String {
        let res = self.get_action();

        res(request, response)
    }

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
