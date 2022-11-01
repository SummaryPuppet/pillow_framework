use sunny::router::{response::Response, Router};

fn main() {
    let mut app = Router::new();

    app.get(
        String::from("/"),
        Response::view(String::from("index.html")),
    );

    println!("Server on port: http://127.0.0.1:5000");
    app.listen("5000");
}
