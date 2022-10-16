use tokio_tungstenite::connect_async;
use url::Url;

use crate::gateway;


pub struct Client {
    pub gateway: Option<gateway::DiscordGateway>,
}

impl Client {
    pub fn new() -> Self {
        Self { gateway: None }
    }

    pub async fn connect(&mut self) {
        let (ws, _) = connect_async(
            Url::parse("wss://gateway.discord.gg").unwrap()
        ).await.unwrap();
        self.gateway = Some(gateway::DiscordGateway::new(ws));
    }
}