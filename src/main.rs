use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
    sync::mpsc::channel,
};

use anyhow::Result;
use game::{watch, GameMessage};

use crate::{lockfile::get_lockfile_credentials, websocket::run_websocket};

pub mod discord;
pub mod game;
pub mod lockfile;
pub mod valorant;
pub mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = channel();
    watch(tx, get_riot_dir().unwrap().as_path());
    match rx.recv() {
        Ok(message) => match message {
            GameMessage::GameStarted => {
                println!("Game Started!");
                let creds = get_lockfile_credentials().await?;
                run_websocket(creds).await.unwrap();
                println!("Websocket stopped!");
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
