extern crate pillow;

use pillow::http::{response::Response, router::Router};

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view("index"));

    let x = |request: httparse::Request, response: Response| {
        println!("{:#?}", request);
        response.text("hola")
    };

    app.post("/post", x);

    app.listen("5000").await;
}
