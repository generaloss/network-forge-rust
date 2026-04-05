use std::io::{self, Read};
use std::net::{TcpListener, ToSocketAddrs};
use std::thread;

pub struct TCPServer { }

impl TCPServer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn run<A: ToSocketAddrs>(&self, address: A) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                println!("[TCPServer] New connection: {0}", stream.peer_addr().unwrap());

                let mut buffer = [0u8; 1024];

                loop {
                    match stream.read(&mut buffer) {
                        Ok(0) => {
                            println!("[TCPServer] Connection closed by other side");
                            break;
                        }
                        Ok(n) => {
                            println!("[TCPServer] Read: {}", String::from_utf8_lossy(&buffer[..n]));
                        }
                        Err(e) => {
                            eprint!("[TCPServer] Read error: {}", e);
                            break;
                        }
                    }
                }
            }
        });

        Ok(())
    }
}