/* Not yet */
use pillow::{
    http::{request::Request, response::Response},
    routing::router::Router,
    websocket::WebSocket,
};

fn initial_route(_: Request, response: Response) -> String {
    response.text("hello")
}

#[async_std::main]
async fn main() {
    let mut app = Router::new();

    app.get("/", initial_route);

    let ws = WebSocket::new(app);

    ws.io("");

    ws.listen("5000").await;
}
