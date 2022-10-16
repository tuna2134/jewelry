use jewerly::client::Client;


#[tokio::main]
async fn main() {
    let mut client = Client::new();
    client.connect().await;
}