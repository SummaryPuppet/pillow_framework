use pillow::http::*;

use macros::routes::users_route;

#[controller(method = "GET", path = "/")]
pub fn index() -> Response {
    Response::html("index")
}

#[controller(method = "POST", path = "/")]
pub fn index_post() -> Response {
    let body = request.body().clone();

    match body {
        Body::JSON(value) => Response::json(value),
        _ => Response::text("hello"),
    }
}

#[controller(method = "GET", path = "/ws")]
pub fn websocket() {
    println!("{:#?}", request);

    Response::text("hello")
}

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.public();
    router.assets();

    router.add_route(route!(index {}));
    router.add_route(route!(index_post {}));
    router.add_route(route!(users_route {}));
    router.add_route(route!(websocket {}));

    let server = Server::new().unwrap();

    server.run(router).await;
}
