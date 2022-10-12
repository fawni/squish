use config::Config;
use inquire::{Select, Text};
use reqwest::Client;
use std::{error::Error, fs, process::Command};
use twitch::{get_live_channels, get_user_id};

mod config;
mod twitch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if fs::metadata(confy::get_configuration_file_path("squish", "config")?).is_err() {
        let username = Text::new("Username").prompt()?;
        let client_id = Text::new("Client ID").prompt()?;
        let access_token = Text::new("Access Token").prompt()?;
        let cfg = Config {
            username,
            client_id,
            access_token,
        };
        confy::store("squish", "config", cfg)?;
        return Ok(());
    };

    let cfg: Config = confy::load("squish", "config")?;

    let client = Client::new();
    let id = get_user_id(&client, &cfg).await?;
    let live_channels = get_live_channels(&client, &cfg, id).await?;

    let mut chans = vec![];
    for channel in &live_channels {
        chans.push(&channel.user_name);
    }
    let channel = Select::new("Pick a channel", chans)
        .without_help_message()
        .with_page_size(10)
        .prompt()?;
    Command::new("mpv")
        .arg(format!("https://twitch.tv/{}", channel))
        .output()?;
    Ok(())
}