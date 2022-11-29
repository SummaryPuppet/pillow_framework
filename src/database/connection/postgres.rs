use colored::Colorize;
use postgres::{Client, NoTls};

pub struct Postgres {
    _url: String,
    pub connection: Client,
}

impl Postgres {
    pub fn new() -> Postgres {
        // postgresql:://username:password@localhost/database
        let url = "postgresql://postgres:password@localhost/test";
        let client = match Client::connect(url, NoTls) {
            Ok(client) => client,
            Err(error) => panic!("{}: {}", "Postgres".red(), error),
        };

        Postgres {
            _url: url.to_string(),
            connection: client,
        }
    }
}

impl Postgres {
    /// CREATE TABLE
    /// # Examples
    /// ```rust
    /// use pillow::database::connection::postgres::Postgres;
    ///
    /// let mut postgres = Postgres::new();
    ///
    /// postgres.create_table("author","(
    ///     id  SERIAL PRIMARY KEY,
    ///     name VARCHAR NOT NULL,
    ///     country VARCHAR NOT NULL
    /// )");
    /// ```
    pub fn create_table(&mut self, name_table: &str, params: &str) {
        let q = format!("CREATE TABLE IF NOT EXISTS {} {}", name_table, params);

        match &self.connection.batch_execute(q.as_str()) {
            Ok(_) => {}
            Err(error) => panic!("{}: {}", "Postgres".red(), error),
        }
    }
}
