use tokio::net::TcpListener;
use std::sync::Arc;
use crate::tcp_connection::TCPConnection;
use crate::tcp_connection_codec::Codec;

pub struct TCPServer<C: Codec> {
    codec: Arc<C>,
}

impl<C: Codec> TCPServer<C> {
    pub fn new(codec: C) -> Self {
        Self {
            codec: Arc::new(codec),
        }
    }

    pub async fn run<F>(&self, addr: &str, handler: F)
    where
        F: Fn(TCPConnection, Vec<u8>) + Send + Sync + 'static,
    {
        let listener = TcpListener::bind(addr).await.unwrap();
        let handler = Arc::new(handler);

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let connection = TCPConnection::new(stream);
            let codec = self.codec.clone();
            let handler = handler.clone();

            tokio::spawn(async move {
                let mut buffer = Vec::new();
                let mut temp = [0u8; 1024];

                loop {
                    let n = connection.read(&mut temp).await.unwrap();
                    if n == 0 {
                        break;
                    }

                    buffer.extend_from_slice(&temp[..n]);

                    while let Some(packet) = codec.try_decode(&mut buffer) {
                        handler(connection.clone(), packet);
                    }
                }
            });
        }
    }
}