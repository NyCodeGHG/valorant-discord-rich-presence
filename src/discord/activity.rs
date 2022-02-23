use std::num::NonZeroU32;

use discord_sdk::activity::{ActivityArgs, ActivityBuilder, Assets};

use crate::valorant::game_state::{
    GameMode::{Competitive, CustomGame, Deathmatch, Replication, SpikeRush, Unknown, Unrated},
    GameState,
    GameStateStatus::{InGame, Menu, PreGame},
};

macro_rules! game_mode {
    ($game_mode:expr) => {
        match $game_mode {
            Competitive => "In a Competitive Match",
            CustomGame => "In a Custom Game",
            Deathmatch => "Playing Deathmatch",
            Replication => "Playing Replication",
            SpikeRush => "Playing Spike Rush",
            Unknown => "Unknown Gamemode",
            Unrated => "Playing Unrated",
        }
    };
}

pub fn build_activity(state: &GameState) -> impl Into<ActivityArgs> {
    let mut activity = ActivityBuilder::default();
    let game_mode = &state.game_mode;
    match state.status {
        InGame => {
            activity = activity
                .details(format!(
                    "{} ({} - {})",
                    game_mode!(game_mode),
                    state.scores.ally_team,
                    state.scores.enemy_team
                ))
                .state(format!("on {}", state.map.display_name));
        }
        PreGame => {
            activity = activity
                .details("In Agent Select")
                .state(game_mode!(game_mode));
        }
        Menu { in_queue } => {
            if in_queue {
                activity = activity.details(match game_mode {
                    CustomGame => "Joining a Custom Game".to_owned(),
                    _ => format!("Queuing {}", game_mode.get_display_name()),
                });
            } else {
                activity = activity.details(match game_mode {
                    CustomGame => "Setting up a Custom Game".to_owned(),
                    _ => format!("Hovering {}", game_mode.get_display_name()),
                });
            }
        }
    };
    if let Some(image_key) = &state.map.image_key {
        activity = activity
            .assets(Assets::default().large(image_key, Some(state.map.display_name.clone())));
    }
    activity.party(
        "uwu",
        NonZeroU32::new(state.party.size),
        NonZeroU32::new(state.party.max_size),
        discord_sdk::activity::PartyPrivacy::Private,
    )
}
