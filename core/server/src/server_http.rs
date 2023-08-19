use std::{
    net::SocketAddr,
    sync::{atomic::AtomicBool, Arc},
};

use pillow_http::Request;
use pillow_routing::MainRouter;
use pillow_ssl::{TcpStream as TcpStreamTLS, TlsAcceptor};

use tokio::{
    io::{AsyncWriteExt, Interest},
    net::TcpListener,
    sync::{watch, Mutex, MutexGuard},
};

/// Server for you app
#[derive(Debug)]
#[allow(dead_code)]
pub struct Server {
    state: watch::Sender<State>,
    /// Address
    addr: [u8; 4],

    /// Port on listen
    port: u16,

    socket_addr: SocketAddr,

    listener: TcpListener,

    tls_acceptor: Option<TlsAcceptor>,

    shutdown: Arc<AtomicBool>,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
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
        Self::new().unwrap()
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
    ///     let server = Server::new().unwrap();
    /// }
    /// ```   
    pub fn new() -> Result<Self, std::io::Error> {
        let server_config = pillow_config::get_config().server();

        let port = server_config.port;
        let addr = server_config.address;

        let tls = match server_config.ssl() {
            Some(_) => Some(TlsAcceptor::new()),
            None => None,
        };

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

            tls_acceptor: tls,
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
        let _url = pillow_config::get_config().server().url;
        self.state.send_replace(State::Listening);

        println!("Listening on http://{}/", &self.socket_addr);

        let router = Arc::new(router);

        let listener = Listener::new(self.listener, router, self.tls_acceptor);

        listener.run().await.unwrap();
    }
}

/// Listener http
struct Listener {
    listener: TcpListener,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    router: Arc<MainRouter>,
}

impl Listener {
    /// Instance of Listener
    ///
    /// # Arguments
    ///
    /// * listener - TcpListener
    pub fn new(listener: TcpListener, router: Arc<MainRouter>, tls: Option<TlsAcceptor>) -> Self {
        Self {
            listener,
            router,
            tls_acceptor: match tls {
                Some(ssl) => Some(Arc::new(ssl)),
                None => None,
            },
        }
    }
}

impl Listener {
    pub async fn run<'a>(&'a self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'a>> {
        match self.tls_acceptor.clone() {
            Some(tls) => self.listen_with_tls(tls).await?,
            None => self.listen().await?,
        };

        Ok(())
    }

    /// Listen Listener
    async fn listen<'a, 'b>(&'a self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'a>> {
        loop {
            match self.listener.accept().await {
                Ok((stream, _client)) => {
                    let router_clone = self.router.clone();
                    let shared_stream = Arc::new(Mutex::new(stream));

                    // let shared_stream_clone = shared_stream.clone();

                    tokio::task::spawn(async move {
                        if let Err(err) =
                            Self::handle_connections(shared_stream, &router_clone).await
                        {
                            eprintln!("{}", err);
                        };
                    });

                    /*
                    tokio::spawn(async move {
                        if let Err(err) = Self::websockets(shared_stream_clone).await {
                            eprintln!("{}", err);
                        };
                    });
                    */
                }

                Err(err) => eprintln!("{}", err),
            };
        }
    }

    async fn listen_with_tls<'a>(
        &'a self,
        tls: Arc<TlsAcceptor>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'a>> {
        while let Ok((stream, _client_addr)) = self.listener.accept().await {
            let tls = tls.clone();
            let stream = tls.accept(stream).await?;
            let router_clone = self.router.clone();

            tokio::task::spawn(async move {
                if let Err(err) = Self::handle_tls_connections(stream, &router_clone).await {
                    eprintln!("{}", err);
                };
            });
        }
        Ok(())
    }

    /*
    async fn websockets(
        stream: Arc<Mutex<tokio::net::TcpStream>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ws = crate::ws::WS::new(stream);

        ws.run().await?;

        Ok(())
    }
     */

    /// Handle new connections
    ///
    /// # Arguments
    ///
    /// * stream - &TcpStream
    /// * router - &MainRouter
    async fn handle_connections(
        stream: Arc<Mutex<tokio::net::TcpStream>>,
        // stream: &mut tokio::net::TcpStream,
        router: &MainRouter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stream = stream.lock().await;

        let ready_readable = stream.ready(Interest::READABLE).await?;
        let ready_writable = stream.ready(Interest::WRITABLE).await?;

        let mut request: Request = Request::new_empty();

        if ready_readable.is_readable() {
            request = Self::read_stream(&stream);
        };

        /*
        if request.uri() == &Uri("/ws".to_string()) {
            println!("ejecting");
            let ws = accept_async(&mut *stream).await?;
            println!(" ws is created");

            let (mut w, mut r) = ws.split();

            while let Some(msg) = r.next().await {
                let msg = msg?;

                match msg {
                    tungstenite::Message::Text(text) => {
                        println!("{:#?}", text);
                        w.send(tungstenite::Message::Text(String::from("hello")))
                            .await?;
                    }

                    tungstenite::Message::Binary(b) => {
                        println!("{:#?}", b);
                    }

                    _ => {}
                }
            }

            return Ok(());
        }
         */

        if ready_writable.is_writable() {
            Self::write_stream(stream, &mut request, &router).await?;
        };

        Ok(())
    }

    async fn handle_tls_connections(
        mut stream: TcpStreamTLS,
        router: &MainRouter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let request = Self::read_stream_tls(&mut stream).await;

        Self::write_stream_tls(stream, &request, router).await?;

        Ok(())
    }

    /// Write data in stream
    ///
    /// # Arguments
    ///
    /// * stream - tokio TcpStream
    /// * request - for Router
    /// * router - MainRouter
    async fn write_stream<'a>(
        // stream: &mut tokio::net::TcpStream,
        mut stream: MutexGuard<'a, tokio::net::TcpStream>,
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

        stream.flush().await?;

        Ok(())
    }

    async fn write_stream_tls(
        mut stream: TcpStreamTLS,
        request: &Request,
        router: &MainRouter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let vec_response = router.routing(request);

        for response in vec_response {
            let headers = format!(
                "{}{}\r\n\r\n",
                response.get_status_line(),
                response.get_headers()
            );
            let body = response.get_body();

            stream.write_all(headers).await;
            stream.write_all(body.to_string()).await;
        }

        stream.flush().await;

        Ok(())
    }

    /// Read data from stream and return a Request
    fn read_stream(stream: &tokio::net::TcpStream) -> Request {
        let mut data = vec![0; 1024];
        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut data) {
            Ok(_) => {
                let parser = Request::from_vec(&data).unwrap();

                return parser;
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
    }

    async fn read_stream_tls(stream: &mut TcpStreamTLS) -> Request {
        let data = vec![0; 1024];
        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        let buf = stream.read(data).await.unwrap();
        Request::from_vec(&buf).unwrap()
    }
}

impl Listener {
    /// Monitor the shutdown signal and close the listener when received
    #[allow(dead_code)]
    async fn monitor_shutdown(mut shutdown_rx: watch::Receiver<State>) {
        // Wait for the shutdown signal
        let _ = shutdown_rx.changed().await;
    }

    /// Wait for the listener to finish
    #[allow(dead_code)]
    async fn await_shutdown(self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
