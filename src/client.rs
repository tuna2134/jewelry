use tokio_tungstenite::connect_async;
use url::Url;

use crate::{
    gateway,
    http,
};


pub struct Client {
    pub gateway: Option<gateway::DiscordGateway>,
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
        let gateway_url = self.http.get_gateway().await;
        let (ws, _) = connect_async(
            Url::parse(&gateway_url).unwrap()
        ).await.unwrap();
        self.gateway = Some(gateway::DiscordGateway::new(ws));
    }

    pub async fn login(&mut self) {
        self.user = Some(self.http.get_current_user().await);
    }

    pub async fn start(&mut self) {
        self.connect().await;
    }
}