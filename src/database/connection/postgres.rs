use std::collections::HashMap;

use postgres::{Client, NoTls};

pub struct Postgres {
    _url: String,
    connection: Client,
}

impl Postgres {
    pub fn new() -> Postgres {
        let url = "";
        let client = Client::connect(url, NoTls).unwrap();

        Postgres {
            _url: url.to_string(),
            connection: client,
        }
    }
}

impl Postgres {
    pub fn create_table(&mut self, name_table: &str, _params: HashMap<String, String>) {
        let q = format!("CREATE TABLE IF NOT EXISTS {} ", name_table);

        match &self.connection.batch_execute(q.as_str()) {
            Ok(_) => {}
            Err(error) => panic!("Postgres: {error}"),
        }
    }
}
