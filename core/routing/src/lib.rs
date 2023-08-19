//! Router for Pillow app
//!
//! ```rust
//! use pillow_routing::router::Router;
//! ```

#![allow(dead_code)]

mod route;
mod router;

pub use route::Route;
pub use router::MainRouter;
