use std::{
    net::SocketAddr,
    sync::{atomic::AtomicBool, Arc},
};

use pillow_http::Request;

use tokio::{
    io::{AsyncWriteExt, Interest},
    net::TcpListener,
    sync::watch,
};

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

    shutdown: Arc<AtomicBool>,
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
        Self::new(3000).unwrap()
    }
}

impl Server {
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
    ///     let server = Server::new(3000).unwrap();
    /// }
    /// ```   
    pub fn new(port: u16) -> Result<Self, std::io::Error> {
        let addr = [127, 0, 0, 1];

        let (state, _) = watch::channel(State::Starting);
        let socket_addr = SocketAddr::from((addr, port.try_into().unwrap()));

        let socket = tokio::net::TcpSocket::new_v4()?;

        #[cfg(not(win))]
        socket.set_reuseaddr(true)?;

        match socket.bind(socket_addr) {
            Ok(_) => {}
            Err(_) => {
                let socket_addr = SocketAddr::from((addr, port + 1));
                socket.bind(socket_addr).unwrap();
            }
        };

        let listener = socket.listen(1024)?;

        let shutdown = Arc::new(AtomicBool::new(false));

        Ok(Self {
            state,
            addr,
            port,
            socket_addr,
            listener,
            shutdown,
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

        let router = Arc::new(router);

        let listener = Listener::new(self.listener, router);

        listener.listen().await.unwrap();
    }
}

/// Listener http
struct Listener {
    listener: TcpListener,
    router: Arc<MainRouter>,
}

impl Listener {
    /// Instance of Listener
    ///
    /// # Arguments
    ///
    /// * listener - TcpListener
    pub fn new(listener: TcpListener, router: Arc<MainRouter>) -> Self {
        Self { listener, router }
    }
}

impl Listener {
    /// Listen Listener
    pub async fn listen<'a, 'b>(
        &'a self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'a>> {
        loop {
            match self.listener.accept().await {
                Ok((mut stream, _client)) => {
                    let router_clone = self.router.clone();

                    tokio::task::spawn(async move {
                        if let Err(err) = Self::handle_connections(&mut stream, &router_clone).await
                        {
                            eprintln!("{}", err);
                        };
                    });
                }
                Err(err) => eprintln!("{}", err),
            };
        }
    }

    /// Handle new connections
    ///
    /// # Arguments
    ///
    /// * stream - &TcpStream
    /// * router - &MainRouter
    async fn handle_connections(
        stream: &mut tokio::net::TcpStream,
        router: &MainRouter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ready_readable = stream.ready(Interest::READABLE).await?;
        let ready_writable = stream.ready(Interest::WRITABLE).await?;

        let mut request: Request = Request::new_empty();

        if ready_readable.is_readable() {
            request = Self::read_stream(request, &stream);
        };

        if ready_writable.is_writable() {
            Self::write_stream(stream, &mut request, &router).await?;
            stream.flush().await?;
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
    async fn write_stream(
        stream: &mut tokio::net::TcpStream,
        request: &Request,
        router: &MainRouter,
    ) -> Result<(), std::io::Error> {
        let vec_response = router.routing(request);

        for response in vec_response {
            let headers = format!(
                "{}{}\r\n\r\n",
                response.get_status_line(),
                response.get_headers()
            );
            let body = response.get_body();

            stream.write_all(headers.as_bytes()).await?;
            stream.write_all(body.as_bytes()).await?;
        }
        Ok(())
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

impl Listener {
    /// Monitor the shutdown signal and close the listener when received
    async fn monitor_shutdown(mut shutdown_rx: watch::Receiver<State>) {
        // Wait for the shutdown signal
        let _ = shutdown_rx.changed().await;
    }

    /// Wait for the listener to finish
    async fn await_shutdown(self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
