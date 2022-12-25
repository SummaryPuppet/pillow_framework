use crate::routing::router::Router;

pub struct WebSocket {
    server: Router,
}

pub struct Socket {}

impl WebSocket {
    pub fn new(mut app: Router) -> WebSocket {
        app.get("/ws", |_, mut response| {
            response.websocket_upgrade_connection()
        });

        WebSocket { server: app }
    }
}

impl WebSocket {
    pub fn io(&self, _event: &str) {}
}

impl WebSocket {
    pub async fn listen(&self, port: &str) {
        self.server.listen(port).await;
    }
}

impl Socket {
    pub fn new() {}
}

impl Socket {
    pub fn on(&self) {}

    pub fn emit(&self) {}
}
