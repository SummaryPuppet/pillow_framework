mod router;

use router::Router;

fn main() {
    let mut app = Router::new();

    app.get("/", |_, response| response.view(String::from("index.html")));

    app.get("/texto", |_, response| response.text(String::from("hola")));

    let json = r#"
        {
            "name": "SummaryPuppet",
            "age": 18
        }
        "#;

    app.get("/j", |_, response| response.json(json));

    app.post("/jose", |_, response| response.text(String::from("hola")));

    println!("Server on port: http://127.0.0.1:5000");
    app.listen("5000");
}
