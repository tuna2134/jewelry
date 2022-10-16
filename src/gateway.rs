use tokio_tungstenite::{
    WebSocketStream,
    MaybeTlsStream,
};

use tokio::net::TcpStream;


pub struct DiscordGateway {
    pub ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl DiscordGateway {
    pub fn new(ws: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws }
    }
}