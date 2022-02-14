use std::{path::Path, sync::mpsc::channel};

use game::GameWatcher;

pub mod game;
pub mod lockfile;

#[tokio::main]
async fn main() {
    let (tx, rx) = channel();
    let _ = GameWatcher::new(
        tx,
        Path::new("C:/Users/chloe/AppData/Local/Riot Games/Riot Client/Config/"),
    );
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
