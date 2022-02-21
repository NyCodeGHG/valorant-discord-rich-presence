use std::{
    env::{self, VarError},
    num::NonZeroU32,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use game::{watch, GameMessage};

use crate::{lockfile::get_lockfile_credentials, valorant::websocket::receive_websocket_events};

pub mod discord;
pub mod game;
pub mod lockfile;
pub mod valorant;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = channel();
    watch(tx, get_riot_dir().unwrap().as_path());
    match rx.recv() {
        Ok(message) => match message {
            GameMessage::GameStarted => {
                println!("Game Started!");
                let creds = get_lockfile_credentials().await?;
                receive_websocket_events(creds).await.unwrap();
            }
            GameMessage::GameStopped => {
                println!("Game Stopped!");
            }
        },
        Err(e) => {
            println!("watch error: {:?}", e);
        }
    }
    Ok(())
}

fn get_riot_dir() -> Result<PathBuf, VarError> {
    let local_app_data = env::var("LOCALAPPDATA")?;
    Ok(Path::new(&local_app_data).join("Riot Games/Riot Client/Config/"))
}
