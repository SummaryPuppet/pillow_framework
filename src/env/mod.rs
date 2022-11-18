//! Eviroment Variables
//!
//! Get variables from .env file

use dotenv::dotenv;
use std::{collections::HashMap, env};

/// Eviroment variables implementation
#[derive(Debug)]
pub struct Env {
    pub variables: HashMap<String, String>,
}

impl Env {
    /// Create a new Env with all variables in .env file
    pub fn new() -> Env {
        dotenv().ok();

        let mut variables: HashMap<String, String> = HashMap::new();

        for (k, v) in env::vars() {
            variables.insert(k, v);
        }

        Env { variables }
    }
}

impl Env {
    /// Returns String value from .env
    ///
    /// # Arguments
    ///
    /// * `var` - name of variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow::env::Env
    ///
    /// let port = Env::get_env_var("".to_string());
    /// ```
    pub fn get_env_var(var: String) -> String {
        dotenv().ok();

        match env::var(&var) {
            Ok(v) => v,
            Err(error) => panic!("${var} is not set: {error}"),
        }
    }
}
