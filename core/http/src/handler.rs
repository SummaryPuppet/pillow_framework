use crate::{Request, Response};

pub trait Handler {
    fn handler(request: &Request) -> Response
    where
        Self: std::fmt::Debug + Sized + Send + Sync;
}
