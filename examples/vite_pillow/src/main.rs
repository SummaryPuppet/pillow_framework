use pillow::http::Router;

#[tokio::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view("index"));

    app.listen("5000").await
}
