use jewelry::Client;

use std::env;


#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let mut client = Client::new(token.as_str());
    client.start().await;
    match client.user {
        Some(user) => println!("Logged in as {}", user["username"]),
        None => println!("Failed to log in"),
    }
}
