use crate::{request::Request, response::Response};

pub trait Handler {
    fn handle(_request: Request, response: Response) -> String;
}
