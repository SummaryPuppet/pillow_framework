// use pillow::http::{request::Request, response::Response};
use pillow_macros::controller;

#[controller]
fn index(_: _, response: _) -> String {
    response.text("hello")
}

fn main() {}
