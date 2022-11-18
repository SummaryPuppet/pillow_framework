use mysql::prelude::*;
use mysql::*;

pub struct Mysql {
    url: String,
    connection: PooledConn,
}

impl Mysql {
    pub fn new() -> Mysql {
        let url = "";
        let pool = Pool::new(url).unwrap();
        let conn = pool.get_conn().unwrap();

        Mysql {
            url: url.to_string(),
            connection: conn,
        }
    }
}

impl Mysql {
    pub fn query(&mut self, mut q: &str) -> Vec<String> {
        match self.connection.query(&mut q) {
            Ok(vec) => vec,
            Err(error) => panic!("{error}"),
        }
    }
}
