use std::{net::SocketAddr, sync::Arc};

use pillow_http::Request;

use tokio::{io::Interest, net::TcpListener, sync::watch};

use crate::MainRouter;

/// Server for you app
#[derive(Debug)]
pub struct Server {
    state: watch::Sender<State>,
    /// Address
    addr: [u8; 4],

    /// Port on listen
    port: u16,

    socket_addr: SocketAddr,

    listener: TcpListener,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum State {
    Starting,
    Listening,
    Shutdown,
}

impl std::default::Default for Server {
    /// Instance of Server
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main(){
    ///     let server = Server::default();
    /// }
    /// ```
    fn default() -> Self {
        Self::new_port_default().unwrap()
    }
}

impl Server {
    /// Instance of Server
    /// with default port in 5000
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main(){
    ///     let server = Server::new_port_default().unwrap();
    /// }
    /// ```   
    pub fn new_port_default() -> Result<Self, std::io::Error> {
        let addr = [127, 0, 0, 1];
        let port = 5000;

        let (state, _) = watch::channel(State::Starting);

        let socket_addr = SocketAddr::from((addr, port));

        let socket = tokio::net::TcpSocket::new_v4().unwrap();

        #[cfg(not(windows))]
        socket.set_reuseaddr(true)?;

        match socket.bind(socket_addr) {
            Ok(_) => {}
            Err(_) => {
                let socket_addr = SocketAddr::from((addr, port + 1));
                socket.bind(socket_addr).unwrap();
            }
        };

        let listener = socket.listen(1024)?;

        Ok(Self {
            state,
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main(){
    ///     let server = Server::new_port_default().unwrap();
    /// }
    /// ```   
    pub fn new_port_personalized(port: u16) -> Result<Self, std::io::Error> {
        let addr = [127, 0, 0, 1];

        let (state, _) = watch::channel(State::Starting);
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
            state,
            addr,
            port,
            socket_addr,
            listener,
        })
    }

    /// Reference of add
    pub fn addr(&self) -> &[u8; 4] {
        &self.addr
    }

    /// Reference of port
    pub fn port(&self) -> &u16 {
        &self.port
    }

    /// Reference of socket_addr
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async main(){
    ///     let mut router = MainRouter::new();
    ///     let server = Server::default();
    ///     server.run(router).await;
    /// }
    /// ```
    pub async fn run(self, router: MainRouter) {
        self.state.send_replace(State::Listening);

        println!("Listening on http://{}/", &self.socket_addr);

        let listener = Listener::new(self.listener);

        let router_arc = Arc::new(router);

        tokio::select! {
            result = listener.listen(&router_arc) => {
                if let Err(err) = result {
                    println!("{err}")
                }
            }
        }
    }
}

/// Listener http
struct Listener {
    listener: TcpListener,
}

impl Listener {
    /// Instance of Listener
    ///
    /// # Arguments
    ///
    /// * listener - TcpListener
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }
}

impl Listener {
    /// Listen Listener
    pub async fn listen<'a, 'b>(
        &'a self,
        router: &Arc<MainRouter>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'a>> {
        loop {
            let (stream, _client_addr) = self.listener.accept().await?;

            let router_clone = router.clone();

            println!("hello");
            tokio::task::spawn(async move {
                match Self::handle_connections(&stream, &router_clone).await {
                    Ok(_) => {}
                    Err(err) => panic!("{}", err),
                };
            });
        }
    }

    /// Handle new connections
    ///
    /// # Arguments
    ///
    /// * stream - &TcpStream
    /// * router - &MainRouter
    async fn handle_connections(
        stream: &tokio::net::TcpStream,
        router: &MainRouter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ready_readable = stream.ready(Interest::READABLE).await?;
        let ready_writable = stream.ready(Interest::WRITABLE).await?;

        let mut request: Request = Request::new_empty();
        if ready_readable.is_readable() {
            request = Self::read_stream(request, &stream);
        };

        if ready_writable.is_writable() {
            Self::write_stream(&stream, &request, &router);
        };

        Ok(())
    }

    /// Write data in stream
    ///
    /// # Arguments
    ///
    /// * stream - tokio TcpStream
    /// * request - for Router
    /// * router - MainRouter
    fn write_stream(stream: &tokio::net::TcpStream, request: &Request, router: &MainRouter) {
        let response = router.routing(&request);

        match stream.try_write(response.to_string().as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };
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
