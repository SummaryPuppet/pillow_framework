pub struct Cors {
    orgins: Vec<String>,
}

impl Cors {
    pub fn new_all() -> Self {
        Self {
            orgins: vec!["*".to_string()],
        }
    }

    pub fn new_whitelist(whitelist: Vec<String>) -> Self {
        Self { orgins: whitelist }
    }
}
