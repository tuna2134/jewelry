use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: bool,
    system: bool,
    mfa_enabled: bool,
    banner: Option<String>,
    accent_color: Option<i32>,
    locale: String,
    verified: bool,
    email: Option<String>,
    flags: i32,
    premium_type: i32,
    public_flags: i32,
}