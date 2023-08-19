use pillow::http::*;

#[controller(method = "GET", path = "/")]
fn index() {
    Response::text("hello world")
}

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.add_route(route!(index {}));

    let server = Server::default();

    server.run(router).await;
}
