//! Pillow a web framework
//!
//! # Example
//!
//! Run
//!
//! ```bash
//! cargo add pillow
//! ```
//!
//! In src/main.rs
//!
//! ```rust
//! use pillow::http::router::Router;
//!
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
//!     app.listen("5000");
//! }
//! ```
//!
//! cargo run
//!
//! ```bash
//! $ Server on 127.0.0.1:5000
//! ```

/// CLI for pillow app
pub mod cli;
/// Env
pub mod env;
/// Http
pub mod http;
