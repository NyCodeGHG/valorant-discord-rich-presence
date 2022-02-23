use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
    sync::mpsc::channel as std_channel,
    time::Duration,
};

use anyhow::Result;
use async_recursion::async_recursion;
use game::{watch, GameMessage};
use lazy_static::lazy_static;
use lockfile::RiotCredentials;
use reqwest::Client;
use tokio::sync::mpsc::channel;

use crate::{
    discord::{activity::build_activity, DiscordPresence},
    lockfile::get_lockfile_credentials,
    valorant::websocket::receive_websocket_events,
};

pub mod discord;
pub mod game;
pub mod lockfile;
pub mod valorant;

const DISCORD_APP_ID: i64 = 944668216486154291;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = std_channel();
    watch(tx, get_riot_dir().unwrap().as_path());
    loop {
        match rx.recv() {
            Ok(message) => match message {
                GameMessage::GameStarted => {
                    println!("Game Started!");
                    let creds = get_lockfile_credentials().await?;
                    wait_until_server_ready(&creds, Duration::from_millis(500)).await;
                    let (sender, mut receiver) = channel(128);
                    receive_websocket_events(sender, creds).await.unwrap();
                    let presence = DiscordPresence::new(DISCORD_APP_ID).await;
                    while let Some(state) = receiver.recv().await {
                        println!("{:#?}", state);
                        let activity = build_activity(&state);
                        presence.discord.update_activity(activity).await.unwrap();
                    }
                    println!("Disconnected from websocket.");
                }
                GameMessage::GameStopped => {
                    println!("Game Stopped!");
                }
            },
            Err(e) => {
                println!("watch error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

#[async_recursion]
async fn wait_until_server_ready(creds: &RiotCredentials, delay: Duration) {
    lazy_static! {
        static ref CLIENT: Client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
    }
    let result = CLIENT
        .get(format!("https://127.0.0.1:{}/help", creds.port))
        .basic_auth("riot", Some(&creds.password))
        .send()
        .await
        .unwrap()
        .text()
        .await;
    if let Ok(text) = result {
        if text.contains("OnJsonApiEvent_chat_v4_presences") {
            return;
        }
    }
    tokio::time::sleep(delay).await;
    println!("Server is not ready yet. Retrying...");
    wait_until_server_ready(creds, delay * 2).await;
}

fn get_riot_dir() -> Result<PathBuf, VarError> {
    let local_app_data = env::var("LOCALAPPDATA")?;
    Ok(Path::new(&local_app_data).join("Riot Games/Riot Client/Config/"))
}
