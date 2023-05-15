use pillow::http::{MainRouter, Response, Server};

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.get("/", |_| Response::view("index"));

    router.get("/create>", |request| {
        println!("{}", request);
        Response::redirect("/")
    });

    let server = Server::new_port_default().unwrap();

    server.run(router).await;
}
