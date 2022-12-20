use colored::Colorize;
use mysql::prelude::*;
use mysql::*;

pub struct Mysql {
    url: String,
    connection: PooledConn,
}

impl Mysql {
    pub fn new() -> Mysql {
        // mysql://root:password@localhost:3307/database_name
        let url = "mysql://root:password@localhost:3307/test";
        let pool = Pool::new(url).unwrap();
        let conn = match pool.get_conn() {
            Ok(conn) => conn,
            Err(error) => panic!("{}: {}", "Mysql".red(), error),
        };

        Mysql {
            url: url.to_string(),
            connection: conn,
        }
    }
}

impl Mysql {
    pub fn create_table(&mut self, name_table: &'static str, params: &'static str) {
        let q = format!("CREATE TABLE {} {}", name_table, params);

        match self.connection.query_drop(q) {
            Ok(_) => {}
            Err(error) => panic!("{} {}", "Mysql".red(), error),
        }
    }

    pub fn query(&mut self, mut q: &str) -> Vec<String> {
        match self.connection.query(&mut q) {
            Ok(vec) => vec,
            Err(error) => panic!("{error}"),
        }
    }
}
