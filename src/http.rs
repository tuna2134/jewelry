use reqwest::{
    RequestBuilder,
    Client,
    Method,
};


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
}