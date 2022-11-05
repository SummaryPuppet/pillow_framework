extern crate pillow;

use pillow::router::Router;
use pillow::serde_json::json;

fn main() {
    let mut app = Router::new();

    app.get("/", |request, mut response| {
        println!("{}", request.path);
        response.view(String::from("index.html"))
    });

    app.get("/post/<jose>", |_, mut response| {
        response.view(String::from("index.html"))
    });

    app.get("/texto", |_, response| response.text(String::from("hola")));

    let json = json!({
        "name": "SummaryPuppet"
    });

    app.get("/j", |_, mut response| response.json(json.to_string()));

    app.post("/jose", |_, response| response.text(String::from("hola")));

    app.listen("5000");
}
