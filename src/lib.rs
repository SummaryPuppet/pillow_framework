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
//! use pillow::routing::router::Router;
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

/// CLI for pillow app
pub mod cli;
/// Database
pub mod database;
/// Env
pub mod env;
/// Errors
mod error;
/// Http
pub mod http;
/// Routing
pub mod routing;
/// Server
mod server;
