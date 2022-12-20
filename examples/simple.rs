extern crate pillow;

use pillow::routing::router::Router;

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view("index"));

    app.get("/about", |_, mut response| response.view("about"));

    app.get("/posts/<id>", |request, mut response| {
        println!("{:#?}", request.parameters);

        response.json_from_str(
            r#"{
        "params": "hola"
    }"#,
        )
    });

    app.get("/dashboard/<name>", |request, response| {
        println!("{:#?}", request.parameters);

        response.text("about")
    });

    app.post("/post/any", |_request, mut response| {
        response.json_from_str(
            r#"
            {
            "name": "James"
            }
            "#,
        )
    });

    app.listen("5000").await;
}
