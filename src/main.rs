use std::{path::Path, sync::mpsc::channel};

use game::{watch, GameMessage};

pub mod game;
pub mod lockfile;

#[tokio::main]
async fn main() {
    let (tx, rx) = channel();
    watch(
        tx,
        Path::new("C:/Users/chloe/AppData/Local/Riot Games/Riot Client/Config/"),
    );
    loop {
        match rx.recv() {
            Ok(message) => match message {
                GameMessage::GameStarted => println!("Game Started!"),
                GameMessage::GameStopped => println!("Game Stopped!"),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
