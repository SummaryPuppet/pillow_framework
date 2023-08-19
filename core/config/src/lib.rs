use std::fs;

use serde::{Deserialize, Serialize};

pub fn get_config() -> Configuration {
    let filename = "pillow.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,

        Err(err) => {
            panic!("Could not read file {} {}", filename, err);
        }
    };
    let configuration: Configuration = match toml::from_str(&contents) {
        Ok(c) => c,

        Err(err) => panic!("Unable to load data from {} {}", filename, err),
    };

    configuration
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    app: Option<App>,

    router: Option<Router>,

    server: Option<Server>,

    database: Option<Database>,
}

impl Configuration {
    pub fn app(self) -> App {
        match self.app {
            Some(app) => app,

            None => App {
                name: String::from("Pillow"),
                debug: true,
            },
        }
    }

    pub fn server(self) -> Server {
        match self.server {
            Some(s) => s,

            None => Server {
                port: 3000,
                url: String::from("http://localhost"),
                address: [127, 0, 0, 1],
                ssl: None,
            },
        }
    }

    pub fn router() {}

    pub fn database(self) -> Database {
        self.database.unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub name: String,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Router {
    directory: Directory,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Directory {
    resourcess: String,
    views: String,
    js: String,
    css: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub port: u16,
    pub url: String,
    pub address: [u8; 4],
    ssl: Option<Ssl>,
}

impl Server {
    pub fn ssl(self) -> Option<Ssl> {
        self.ssl
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ssl {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub connection: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}
