use pillow::http::Router;

#[tokio::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", |_, mut response| response.view("index"));

    app.get("/create?name=AdrianSalcedo", |request, response| {
        println!("{:#?}", request);

        response.text("hello")
    });

    app.listen("5000").await;
}
