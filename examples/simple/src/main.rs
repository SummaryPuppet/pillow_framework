use pillow::{json, routing::router::Router};

#[tokio::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view_hbs("index", json!({})));

    app.get("/about", |_, mut response| response.view("about"));

    app.get("/contacs", |_, mut response| {
        response.json(json!({
            "name": "foo"
        }))
    });

    app.listen("5000").await;
}
