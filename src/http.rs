use reqwest::{
    RequestBuilder,
    Client,
    Method,
};

use serde_json::Value;


pub struct HttpClient {
    pub session: Client,
    baseurl: String,
    token: String,
}

impl HttpClient {
    pub fn new(token: String) -> Self {
        Self {
            session: Client::new(),
            baseurl: "https://discord.com/api/v10".to_string(),
            token,
        }
    }

    pub fn request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        self.session.request(
            method, &format!("{}{}", self.baseurl, endpoint)
        )
        .header("Authorization", &format!("Bot {}", self.token))
    }

    pub async fn get_current_user(&self) -> Value {
        let res = self.request(
            Method::GET, "/users/@me"
        )
        .send()
        .await
        .unwrap();
        res.json().await.unwrap()
    }
}