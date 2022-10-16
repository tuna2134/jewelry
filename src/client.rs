use crate::{
    gateway::DiscordGateway,
    http,
};


pub struct Client {
    pub gateway: Option<DiscordGateway>,
    pub http: http::HttpClient,
    pub token: String,
    pub user: Option<serde_json::Value>,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Self {
            gateway: None,
            token: token.to_string(),
            http: http::HttpClient::new(token.to_string()),
            user: None,
        }
    }

    pub async fn connect(&mut self) {
        let gateway = DiscordGateway::from_client(
            "wss://gateway.discord.gg/?v=10&encoding=json&compress=zlib-stream"
        ).await;
        self.gateway = Some(gateway);
    }

    pub async fn login(&mut self) {
        self.user = Some(self.http.get_current_user().await);
    }

    pub async fn start(&mut self) {
        self.login().await;
        self.connect().await;
    }
}