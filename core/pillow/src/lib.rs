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

pub mod http {
    pub use pillow_http::handler::Handler;
    pub use pillow_http::http_methods::HttpMethods;
    pub use pillow_http::Request;
    pub use pillow_http::Response;
    pub use pillow_routing::MainRouter;
    pub use pillow_routing::Server;
}

#[cfg(feature = "template")]
pub use pillow_templates as template;

#[cfg(feature = "env")]
pub use pillow_env as env;
#[cfg(feature = "fs")]
pub use pillow_fs as fs;
