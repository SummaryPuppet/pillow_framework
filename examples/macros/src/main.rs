use pillow::{
    http::*,
    templates::{Context, Template},
};

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

#[controller(method = "GET", path = "/users/<id>")]
pub fn users() -> Response {
    println!("{:#?}", request.get_param("id"));

    let mut ctx = Context::new();

    ctx.insert("name", "SummaryPuppet");
    ctx.insert("id", &request.get_param("id"));

    Response::view(Template::Tera("users", "tera.html", ctx))
}

#[controller(method = "GET", path = "/ws")]
pub fn websocket() {
    println!("{:#?}", request);

    Response::websocket_upgrade_connection()
}

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.public();
    router.assets();

    router.add_route(route!(index {}));
    router.add_route(route!(index_post {}));
    router.add_route(route!(users {}));
    router.add_route(route!(websocket {}));

    let server = Server::new(3000).unwrap();

    server.run(router).await;
}
