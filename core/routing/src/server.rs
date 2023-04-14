use std::net::SocketAddr;

use pillow_http::Request;

use tokio::{io::Interest, net::TcpListener};

use crate::MainRouter;

/// Server for you app
#[derive(Debug)]
pub struct Server {
    addr: [u8; 4],
    port: u16,
    socket_addr: SocketAddr,
    listener: TcpListener,
}

impl Server {
    /// Instance of Server
    /// with default port in 5000
    pub fn new_port_default() -> Result<Self, std::io::Error> {
        let addr = [127, 0, 0, 1];
        let port = 5000;

        let socket_addr = SocketAddr::from((addr, port));

        let socket = tokio::net::TcpSocket::new_v4().unwrap();
        match socket.bind(socket_addr) {
            Ok(_) => {}
            Err(_) => {
                let socket_addr = SocketAddr::from((addr, port + 1));
                socket.bind(socket_addr).unwrap();
            }
        };

        let listener = socket.listen(1024)?;

        Ok(Self {
            addr,
            port,
            socket_addr,
            listener,
        })
    }

    /// Instance of Server
    ///
    /// # Arguments
    ///
    /// * port - port of your app
    pub fn new_port_personalized(port: u16) -> Result<Self, std::io::Error> {
        let addr = [127, 0, 0, 1];

        let socket_addr = SocketAddr::from((addr, port.try_into().unwrap()));

        let socket = tokio::net::TcpSocket::new_v4().unwrap();
        match socket.bind(socket_addr) {
            Ok(_) => {}
            Err(_) => {
                let socket_addr = SocketAddr::from((addr, port + 1));
                socket.bind(socket_addr).unwrap();
            }
        };

        let listener = socket.listen(1024)?;

        Ok(Self {
            addr,
            port,
            socket_addr,
            listener,
        })
    }

    pub fn addr(&self) -> &[u8; 4] {
        &self.addr
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn socket_addr(&self) -> &SocketAddr {
        &self.socket_addr
    }
}

impl Server {
    /// Run you Server
    ///
    /// # Arguments
    ///
    /// * router - You MainRouter
    pub async fn run(self, router: &MainRouter) {
        println!("Listening on http://{}", self.socket_addr);

        let listener = Listener::new(self.listener);

        listener.listen(&router).await.unwrap();
    }
}

/// Listener http
struct Listener {
    listener: TcpListener,
}

impl Listener {
    /// Instance of Listener
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }

    /// Listener listen new connections
    ///
    /// # Arguments
    ///
    /// * router - MainRouter
    pub async fn listen(
        &self,
        router: &MainRouter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            let (stream, _client_addr) = self.listener.accept().await?;

            let ready_readable = stream.ready(Interest::READABLE).await.unwrap();
            let ready_writable = stream.ready(Interest::WRITABLE).await.unwrap();

            let mut request: Request = Request::new_empty();

            if ready_readable.is_readable() {
                request = Self::read_stream(request, &stream);
            };

            if ready_writable.is_writable() {
                let response = router.routing(&request);

                match stream.try_write(response.to_string().as_bytes()) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e),
                };
            };
        }
    }

    /// Read data from stream and return a Request
    fn read_stream(mut _parser: Request, stream: &tokio::net::TcpStream) -> Request {
        let mut data = vec![0; 1024];
        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut data) {
            Ok(_) => {
                _parser = Request::from_vec(&data).unwrap();

                return _parser;
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
    }
}
