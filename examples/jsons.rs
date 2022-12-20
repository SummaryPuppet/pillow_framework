use pillow::{json, routing::router::Router};

extern crate pillow;

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_request, mut response| {
        let json = json!({
            "name": "SummaryPuppet",
            "age": 18
        });

        response.json(json)
    });

    app.listen("5000").await
}
