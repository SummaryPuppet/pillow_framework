use pillow::{json, routing::router::Router};

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| {
        response.view_hbs("contact", json!({"name": "foo"}))
    });

    app.listen("5000").await
}
