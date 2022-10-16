use jewerly::client::Client;

use std::env;


#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let mut client = Client::new(token.as_str());
    client.login().await;
}