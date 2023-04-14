//! Router for Pillow app
//!
//! ```rust
//! use pillow_routing::router::Router;
//! ```

mod route;
mod router;
pub(crate) mod routes;
mod server;

pub use router::MainRouter;
pub use server::Server;
