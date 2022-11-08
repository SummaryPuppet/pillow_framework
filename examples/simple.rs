extern crate pillow;

use pillow::router::Router;

fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view(String::from("index")));

    app.get("/about", |_, mut response| {
        response.view(String::from("about"))
    });

    app.get("/post/<jose>", |_, mut response| {
        response.view(String::from("index"))
    });

    app.get("/texto", |_, response| response.text(String::from("hola")));

    let json = r#"
        {
            "name": "SummaryPuppet",
            "age": 18
        }
        "#;

    app.get("/j", |_, mut response| response.json(json.to_string()));

    app.post("/jose", |_, response| response.text(String::from("hola")));

    println!("Server on port: http://127.0.0.1:5000");
    app.listen("5000");
}
