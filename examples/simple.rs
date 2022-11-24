extern crate pillow;

use pillow::routing::router::Router;

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view("index"));

    app.get("/posts/<id>", |request, mut response| {
        println!("{:#?}", request.parameters);

        response.json(
            r#"{
        "params": "hola"
    }"#,
        )
    });

    app.get("/dashboard/<name>", |_, response| response.text("about"));

    app.post("/post/any", |_, mut response| {
        response.json(
            r#"
            {
            "name": "James"
            }
            "#,
        )
    });

    app.listen("5000").await;
}
