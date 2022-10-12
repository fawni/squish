use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use crate::config::Config;

#[derive(Deserialize)]
struct UserData {
    id: String,
}

#[derive(Deserialize)]
struct User {
    data: Vec<UserData>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct Channel {
    pub(crate) user_name: String,
    pub(crate) game_name: String,
    pub(crate) title: String,
    pub(crate) viewer_count: u32,
    pub(crate) started_at: String,
    pub(crate) thumbnail_url: String,
}

#[derive(Deserialize)]
struct Followed {
    data: Vec<Channel>,
}

pub(crate) async fn get_user_id(client: &Client, cfg: &Config) -> Result<String, Box<dyn Error>> {
    let client_id = &cfg.client_id;
    let token = &cfg.access_token;
    let user = client
        .get(format!(
            "https://api.twitch.tv/helix/users?login={}",
            cfg.username
        ))
        .header("Authorization", format!("Bearer {}", token))
        .header("Client-id", client_id)
        .send()
        .await?
        .json::<User>()
        .await?;
    let id = user.data[0].id.to_string();

    Ok(id)
}

pub(crate) async fn get_live_channels(
    client: &Client,
    cfg: &Config,
    id: String,
) -> Result<Vec<Channel>, Box<dyn Error>> {
    let live_channels = client
        .get(format!(
            "https://api.twitch.tv/helix/streams/followed?user_id={}",
            id
        ))
        .header("Authorization", format!("Bearer {}", &cfg.access_token))
        .header("Client-id", &cfg.client_id)
        .send()
        .await?
        .json::<Followed>()
        .await?
        .data;

    Ok(live_channels)
}