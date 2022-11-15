extern crate pillow;

use pillow::http::router::Router;

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view("index"));

    app.post("/post", |request, response| {
        println!("{:#?}", request);
        response.text("hoola")
    });

    app.listen("5000").await;
}
