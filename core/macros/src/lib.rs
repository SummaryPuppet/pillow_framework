//! Macros in Pillow
#![allow(dead_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ExprStruct, ItemFn};

mod controller;
mod route;

/// Conver controller in route
///
/// ```rust
/// #[controller(method = "GET", path = "/")]
/// fn index() -> Response {
///     Response::text("hello")
/// }
/// ```
#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);

    let (method, path) = controller::generate_attrs(args);
    controller::generate(input, method, path)
}
/// Conver controller in route
///
/// ```rust
/// #[controller(method = "GET", path = "/")]
/// fn index() -> Response {
///     Response::text("hello")
/// }
/// #[tokio::main]
/// async fn main() {
///     let mut router = MainRouter::new();
///     router.add_route(route!(index {}));
/// }
/// ```
#[proc_macro]
pub fn route(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ExprStruct);

    route::generate(input)
}
