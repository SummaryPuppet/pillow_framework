//! Router for Pillow app
//!
//! ```rust
//! use pillow_routing::router::Router;
//! ```

#![allow(dead_code)]

mod route;
mod router;
mod server;

pub use route::Route;
pub use router::MainRouter;
pub use server::Server;
