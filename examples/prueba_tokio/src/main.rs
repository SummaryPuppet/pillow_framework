use pillow::http::{MainRouter, Response, Server};

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.get("/", |_req| Response::text("poto"));

    let server = Server::new_port_default().unwrap();

    server.run(&router).await;
}
