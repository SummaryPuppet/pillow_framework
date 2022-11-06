use dotenv::dotenv;
use std::env;

pub struct Env {}

impl Env {
    /// Returns String value from .env
    ///
    /// # Arguments
    ///
    /// * `var` - name of variable
    pub fn get_env_var(var: String) -> String {
        dotenv().ok();

        match env::var(&var) {
            Ok(v) => v,
            Err(error) => panic!("${var} is not set: {error}"),
        }
    }
}
