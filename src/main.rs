mod router;

use router::{response, Router};

fn main() {
    let mut app = Router::new();

    /*
    app.get(
        String::from("/hola"),
        response::Response::view(String::from("index.html")),
    );

    app.get(
        String::from("/"),
        response::Response::text(String::from("hola")),
    );
    */

    app.get(
        String::from("/holis"),
        response::Response::json(String::from(
            r#"
            {
           "hola": "hola" 
            }
        "#,
        )),
    );

    app.post(
        String::from("/jose"),
        response::Response::view(String::from("index.html")),
    );

    println!("Server on port: http://127.0.0.1:5000");
    app.listen("5000");
}
