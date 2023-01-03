//! Pillow a web framework
//!
//! Installation
//!
//! ```bash
//! cargo add pillow
//! ```
//!
//! or
//!
//! ```toml
//! [dependencies]
//! pillow = "0.1.6"
//! ```
//!
//! # Example
//!
//! In src/main.rs
//!
//! ```rust
//! use pillow::http::Router;
//!
//! #[async_std::main]
//! fn main(){
//!     let mut app = Router::new();
//!
//!     app.get("/", |_, mut response| response.view("index"));
//!
//!     app.post("/post", |request, response|{
//!         println!("{:#?}", request);
//!         response.text("hello")
//!     })
//!
//!     app.listen("5000").await;
//! }
//! ```
//!
//! cargo run
//!
//! ```bash
//! $ Server on 127.0.0.1:5000
//! ```

/// Database
pub mod database;

pub mod http {
    pub use pillow_http::http_methods::HttpMethods;
    pub use pillow_http::request::Request;
    pub use pillow_http::response::Response;
    pub use pillow_routing::router::Router;
}

pub use pillow_env as env;
pub use pillow_fs as fs;
pub use pillow_http::json;
