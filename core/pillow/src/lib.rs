//! Pillow a web framework
//!
//! # Example
//!
//! In src/main.rs
//!
//! ```rust
//! use pillow::http::*;
//!
//! #[controller(method = "GET", path = "/")]
//! fn index(){
//!     Response::text("hello")
//! }
//!
//! #[tokio::main]
//! fn main(){
//!     let mut router= MainRouter::new();
//!
//!     router.add_route(route!(index {}));
//!
//!     router.post("/post", |request|{
//!         println!("{:#?}", request);
//!         Response::text("hello")
//!     })
//!
//!     let server = Server::default();
//!
//!     server.run(router).await;
//! }
//! ```
//!
//! cargo run
//!
//! ```bash
//! $ Server on 127.0.0.1:5000
//! ```

#[cfg(feature = "http")]
pub mod http {
    pub use pillow_http::handler::Handler;

    pub use pillow_http::http_methods::from_str_to_http_method;
    pub use pillow_http::http_methods::HttpMethods;

    pub use pillow_http::body::Body;

    pub use pillow_http::header::ContentType;
    pub use pillow_http::header::Header;

    pub use pillow_http::Request;
    pub use pillow_http::Response;

    #[cfg(feature = "routing")]
    pub use pillow_routing::MainRouter;

    #[cfg(feature = "routing")]
    pub use pillow_routing::Server;

    #[cfg(feature = "routing")]
    pub use pillow_routing::Route;

    #[cfg(feature = "macros")]
    pub use pillow_macros::controller;

    #[cfg(feature = "macros")]
    pub use pillow_macros::route;
}

#[cfg(feature = "http")]
pub use pillow_http::json;

#[cfg(feature = "templates")]
pub use pillow_templates as templates;

#[cfg(feature = "env")]
pub use pillow_env as env;

#[cfg(feature = "fs")]
pub use pillow_fs as fs;
