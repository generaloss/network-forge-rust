use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct TCPConnection {
    stream: Arc<Mutex<TcpStream>>,
}

impl TCPConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub async fn send(&self, data: &[u8]) -> tokio::io::Result<()> {
        let mut stream = self.stream.lock().await;
        stream.write_all(data).await
    }

    pub async fn read(&self, buf: &mut [u8]) -> tokio::io::Result<usize> {
        let mut stream = self.stream.lock().await;
        stream.read(buf).await
    }
}