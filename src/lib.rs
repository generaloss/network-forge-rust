mod tcp_server;
mod tcp_client;

#[cfg(test)]
mod tests {
    use crate::tcp_server::*;
    use crate::tcp_client::*;

    #[test]
    fn test() {
        let server = TCPServer::new();
        server.run("127.0.0.1:5555").expect("Cannot run server");

        let mut client = TCPClient::new();
        client.connect("127.0.0.1:5555").expect("Cannot connect client");
        client.send("Hello, world!".as_bytes()).expect("Cannot send message");
    }
}
