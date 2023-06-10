//! Http implementation for pillow

#![allow(dead_code)]

pub mod body;
pub mod controller;
mod cors;
pub mod futures_handler;
pub mod handler;
pub mod header;
pub mod http_methods;
pub mod middlewares;
mod params;
mod request;
mod response;
mod uri;

pub use response::static_files;

pub use request::Request;
pub use response::Response;

pub use response::Body as BodyResponse;

pub use serde_json::json;

pub use cors::Cors;
pub use uri::Uri;

pub use response::status_code;
