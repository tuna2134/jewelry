use tokio_tungstenite::{
    WebSocketStream,
    MaybeTlsStream,
    connect_async,
};
use url::Url;

use tokio::net::TcpStream;


pub struct DiscordGateway {
    pub ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl DiscordGateway {
    pub fn new(ws: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws }
    }

    pub async fn from_client(url: &str) -> Self {
        let (ws, _) = connect_async(
            Url::parse(url).unwrap()
        ).await.unwrap();
        Self::new(ws)
    }
}