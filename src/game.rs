use std::{
    path::Path,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

pub fn watch(sender: Sender<GameMessage>, game_dir: &Path) {
    let dir = game_dir.to_path_buf();
    let file = dir.join("lockfile");
    if file.exists() {
        // Game is already running.
        sender.send(GameMessage::GameStarted).expect("Unable to send message");
    }

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
}

#[derive(Debug)]
pub enum GameMessage {
    GameStarted,
    GameStopped,
}
