use tokio_tungstenite::connect_async;
use url::Url;
use reqwest::Method;

use crate::gateway;
use crate::http;


pub struct Client {
    pub gateway: Option<gateway::DiscordGateway>,
    pub http: http::HttpClient,
    pub token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Self {
            gateway: None,
            token: token.to_string(),
            http: http::HttpClient::new(token.to_string()),
        }
    }

    pub async fn connect(&mut self) {
        let (ws, _) = connect_async(
            Url::parse("wss://gateway.discord.gg").unwrap()
        ).await.unwrap();
        self.gateway = Some(gateway::DiscordGateway::new(ws));
    }

    pub async fn login(&mut self) {
        let res = self.http.request(
            Method::GET, "/users/@me"
        )
        .header("Authorization", &format!("Bot {}", self.token))
        .send()
        .await
        .unwrap();
        println!("{:?}", res);
    }

    pub async fn start(&mut self) {
        self.connect().await;
    }
}