use std::{
    path::Path,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

pub struct GameWatcher;

impl GameWatcher {
    pub fn new(sender: Sender<GameMessage>, game_dir: &Path) -> GameWatcher {
        let dir = game_dir.to_path_buf();
        thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();
            watcher.watch(dir, RecursiveMode::NonRecursive).unwrap();
            loop {
                let event = match rx.recv() {
                    Ok(event) => event,
                    Err(e) => {
                        println!("error: {}", e);
                        continue;
                    }
                };
                match event {
                    DebouncedEvent::Create(path) => {
                        if path.ends_with("lockfile") {
                            sender
                                .send(GameMessage::GameStarted)
                                .expect("Unable to send message");
                        }
                    }
                    DebouncedEvent::NoticeRemove(path) => {
                        if path.ends_with("lockfile") {
                            sender
                                .send(GameMessage::GameStopped)
                                .expect("Unable to send message");
                        }
                    }
                    _ => continue,
                };
            }
        });
        GameWatcher {}
    }
}

#[derive(Debug)]
pub enum GameMessage {
    GameStarted,
    GameStopped,
}
