use discord_game_sdk::Activity;

use super::DiscordAction;

#[derive(Debug)]
pub struct UpdateActivityAction {
    activity: Activity,
}

impl DiscordAction for UpdateActivityAction {
    fn execute(&self, discord: &mut discord_game_sdk::Discord<()>) {
        discord.update_activity(&self.activity, |_, _| {});
    }
}

impl UpdateActivityAction {
    pub fn new(activity: Activity) -> UpdateActivityAction {
        UpdateActivityAction { activity }
    }
}
