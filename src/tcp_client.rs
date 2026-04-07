use tokio::net::TcpStream;
use std::sync::Arc;
use crate::tcp_connection::TCPConnection;
use crate::tcp_connection_codec::Codec;

pub struct TCPClient<C: Codec> {
    codec: Arc<C>,
}

impl<C: Codec> TCPClient<C> {
    pub fn new(codec: C) -> Self {
        Self {
            codec: Arc::new(codec),
        }
    }

    pub async fn connect<F>(&self, addr: &str, handler: F) -> TCPConnection
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        let stream = TcpStream::connect(addr).await.unwrap();
        let connection = TCPConnection::new(stream);

        let codec = self.codec.clone();
        let handler = Arc::new(handler);
        let connection_clone = connection.clone();

        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut temp = [0u8; 1024];

            loop {
                let n = connection_clone.read(&mut temp).await.unwrap();
                if n == 0 {
                    break;
                }

                buffer.extend_from_slice(&temp[..n]);

                while let Some(packet) = codec.try_decode(&mut buffer) {
                    handler(packet);
                }
            }
        });

        connection
    }

    pub async fn send(&self, connection: &TCPConnection, data: &[u8]) {
        let encoded = self.codec.encode(data);
        let _ = connection.send(&encoded).await;
    }
}