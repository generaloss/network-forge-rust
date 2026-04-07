use crate::tcp_client::TCPClient;
use crate::tcp_connection_codec::{PacketCodec, StreamCodec};
use crate::tcp_server::TCPServer;

mod tcp_server;
mod tcp_client;
mod tcp_connection;
mod tcp_connection_codec;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::test();
    }
}

#[tokio::main]
async fn test() {
    let server = TCPServer::new(PacketCodec);

    tokio::spawn(async move {
        server.run("127.0.0.1:9000", |connection, data| {
            println!("server got: {:?}", data);
            tokio::spawn(async move {
                let _ = connection.send(b"pong").await;
            });
        }).await;
    });

    let client = TCPClient::new(PacketCodec);

    let connection = client.connect("127.0.0.1:9000", |data| {
        println!("client got: {:?}", data);
    }).await;

    client.send(&connection, b"ping").await;
}