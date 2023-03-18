use config::Config;
use inquire::{Select, Text};
use std::{process::Command, thread};
use twitch::{Token, User};

mod config;
mod twitch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !confy::get_configuration_file_path("squish", "squish")?.exists() {
        let username = Text::new("Username").prompt()?;
        let client_id = Text::new("Client ID").prompt()?;
        let access_token = Token::generate(&client_id)?;

        confy::store(
            "squish",
            "squish",
            Config {
                username,
                client_id,
                access_token,
            },
        )?;
    };

    let config = confy::load::<Config>("squish", "squish")?;
    if !Token::validate(&config.access_token, &config.username).await? {
        confy::store(
            "squish",
            "squish",
            Config {
                access_token: Token::generate(&config.client_id)?,
                ..config.clone()
            },
        )?;
    }

    let channels = User::live_channels(&config.username, &config.client_id, &config.access_token)
        .await?
        .into_iter()
        .map(|channel| channel.user_name)
        .collect::<Vec<String>>();

    let channel = Select::new("Pick a channel", channels)
        .without_help_message()
        .with_page_size(10)
        .prompt()?;
    let channel_url = format!("https://twitch.tv/{channel}");

    let mpv = thread::spawn(move || {
        Command::new("mpv").arg(channel_url).output().unwrap();
    });

    let chatterino = thread::spawn(move || {
        Command::new("chatterino")
            .args(["-c", &channel])
            .output()
            .unwrap();
    });

    mpv.join().unwrap();
    chatterino.join().unwrap();

    Ok(())
}
