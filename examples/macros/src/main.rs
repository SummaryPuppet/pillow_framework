use pillow::http::*;
use pillow_macros::{controller, route};

#[controller(method = "GET", path = "/")]
pub fn index() -> Response {
    println!("{:#?}", request);

    Response::text("Wey")
}

#[controller(method = "GET", path = "/users/<poto>")]
pub fn users() -> Response {
    Response::text("users")
}

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.add_route(route!(index {}));
    router.add_route(route!(users {}));

    let server = Server::default();

    server.run(router).await;
}
