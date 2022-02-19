use std::{
    env::{self, VarError},
    path::{Path, PathBuf}, sync::mpsc::channel,
};

use anyhow::Result;
use game::{GameMessage, watch};

pub mod game;
pub mod lockfile;
pub mod valorant;
pub mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = channel();
    watch(tx, get_riot_dir().unwrap().as_path());
    loop {
        match rx.recv() {
            Ok(message) => match message {
                GameMessage::GameStarted => println!("Game Started!"),
                GameMessage::GameStopped => {
                    println!("Game Stopped!");
                    break;
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

fn get_riot_dir() -> Result<PathBuf, VarError> {
    let local_app_data = env::var("LOCALAPPDATA")?;
    Ok(Path::new(&local_app_data).join("Riot Games/Riot Client/Config/"))
}
