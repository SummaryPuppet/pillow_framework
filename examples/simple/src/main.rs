use pillow::{
    http::{Handler, Request, Response, Router},
    json,
};

fn middle(request: &Request, _: &Response) {
    println!("{:#?} {:#?}", request.method, request.path);
}

#[derive(Debug)]
struct Ctrl {}

impl Handler for Ctrl {
    fn handle(_request: Request, response: Response) -> String {
        response.text("hello")
    }
}

#[tokio::main]
async fn main() {
    let mut app = Router::new();

    app.add_middleware(middle);

    app.get("/", |_, mut response| response.view_hbs("index", json!({})));

    app.get("/struct", Ctrl::handle);

    app.get_struct("/struct", Ctrl {});

    app.get("/about", |_, mut response| response.view("about"));

    app.get("/contacs", |_, mut response| {
        response.json(json!({
            "name": "foo"
        }))
    });

    app.listen_tokio("5000").await;
}
