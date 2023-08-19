use std::{fs, ops::Deref};

use native_tls::{Identity, TlsAcceptor as NativeTlsAcceptor, TlsStream as NativeTlsStream};
use pillow_config::get_config;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream as TokioTcpStream,
};
use tokio_native_tls::{TlsAcceptor as TokioTlsAcceptor, TlsStream};

#[derive(Debug, Clone)]
pub struct TlsAcceptor {
    tls: TokioTlsAcceptor,
}

impl Deref for TlsAcceptor {
    type Target = TokioTlsAcceptor;

    fn deref(&self) -> &Self::Target {
        &self.tls
    }
}

impl TlsAcceptor {
    pub fn new() -> TlsAcceptor {
        let ssl_config = get_config()
            .server()
            .ssl()
            .expect("Add [server.ssl] in pillow.toml");

        let cert = fs::read(ssl_config.cert).expect("Don't read certificate");
        let key = fs::read(ssl_config.key).expect("Don't read key");

        let identity = Identity::from_pkcs8(&cert, &key).expect("Failed to load TLS identity");
        let acceptor = TokioTlsAcceptor::from(NativeTlsAcceptor::new(identity).unwrap());

        Self { tls: acceptor }
    }

    pub async fn accept(
        &self,
        stream: TokioTcpStream,
    ) -> Result<TcpStream, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let tls_stream = self.tls.accept(stream).await?;

        let s = TcpStream::new(tls_stream);

        Ok(s)
    }
}

pub struct TcpStream {
    stream: TlsStream<TokioTcpStream>,
}

impl Deref for TcpStream {
    type Target = TlsStream<TokioTcpStream>;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl TcpStream {
    pub fn new(stream: TlsStream<TokioTcpStream>) -> Self {
        Self { stream }
    }

    pub async fn read(
        &mut self,
        mut buf: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let _ = &self.stream.read_buf(&mut buf).await?;

        Ok(buf)
    }

    pub async fn write_all(&mut self, data: String) {
        let _ = &self.stream.write_all(data.as_bytes()).await;
    }

    pub async fn flush(&mut self) {
        let _ = &self.stream.flush().await;
    }
}
