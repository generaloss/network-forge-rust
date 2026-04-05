use std::net::{TcpStream, ToSocketAddrs};
use std::io;
use std::io::Write;

pub struct TCPClient {
    stream: Option<TcpStream>
}

impl TCPClient {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn connect<A: ToSocketAddrs>(&mut self, address: A) -> io::Result<()> {
        let stream = TcpStream::connect(address)?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn send(&self, buffer: &[u8]) -> io::Result<()> {
        let mut stream = self.stream.as_ref().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "[TCPClient] TcpStream closed")
        })?;

        stream.write_all(buffer)?;
        Ok(())
    }
}