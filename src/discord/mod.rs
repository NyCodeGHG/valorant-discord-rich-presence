use std::fmt::Debug;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;

use anyhow::Result;
use discord_game_sdk::Discord;

pub mod actions;

pub struct DiscordPresence {
    tx: Sender<Box<dyn DiscordAction>>,
}
pub trait DiscordAction: Sync + Send + Debug {
    fn execute(&self, discord: &mut Discord<()>);
}

impl DiscordPresence {
    pub fn new(client_id: i64) -> Result<DiscordPresence> {
        let (tx, rx) = channel::<Box<dyn DiscordAction>>();
        let presence = DiscordPresence { tx };
        thread::spawn(move || {
            let mut discord: Discord<()> =
                Discord::new(client_id).expect("Unable to create discord client");
            loop {
                let callback = discord.run_callbacks();
                println!("Running callback.");
                if let Err(e) = callback {
                    panic!("{}", e);
                }
                let action = match rx.try_recv() {
                    Ok(action) => action,
                    Err(TryRecvError::Empty) => continue,
                    Err(TryRecvError::Disconnected) => {
                        println!("Closed Discord Client.");
                        break;
                    }
                };
                action.execute(&mut discord);
            }
        });
        Ok(presence)
    }

    pub fn queue_action<A: DiscordAction + 'static>(&self, action: A) -> Result<()> {
        self.tx.send(Box::new(action))?;
        Ok(())
    }

    pub fn close(self) {}
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use discord_game_sdk::Activity;

    use super::{actions::UpdateActivityAction, DiscordPresence};

    #[tokio::test]
    async fn simple_presence() {
        let client = DiscordPresence::new(944668216486154291).unwrap();
        let activity = Activity::empty()
            .with_state("on Ascent")
            .with_details("In a Competitive Match (7 - 3)")
            .with_large_image_key("ascent")
            .with_large_image_tooltip("Ascent")
            .with_party_amount(2)
            .with_party_capacity(5)
            .with_instance(true)
            .to_owned();
        client
            .queue_action(UpdateActivityAction::new(activity))
            .unwrap();
        thread::sleep(Duration::from_secs(1000));
        client.close();
        thread::sleep(Duration::from_secs(2));
    }
}
