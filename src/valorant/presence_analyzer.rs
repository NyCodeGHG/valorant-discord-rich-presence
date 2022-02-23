use serde::{Deserialize, Serialize};

use super::{
    game_state::{GameMode, GameState, GameStateStatus, Party, Scores},
    presence::ParsedPresence,
};
pub fn analyze_presence(presence: &ParsedPresence) -> GameState {
    let game_mode = determine_game_mode(presence);

    let party = Party {
        size: presence.party_size,
        max_size: presence.max_party_size,
    };

    let scores = Scores {
        ally_team: presence.party_owner_match_score_ally_team,
        enemy_team: presence.party_owner_match_score_enemy_team,
    };

    let status = determine_status(presence);

    let map = determine_map(&presence.match_map);

    GameState {
        game_mode,
        party,
        scores,
        status,
        map,
    }
}

macro_rules! map_key {
    ($single:expr) => {
        map_key!($single, $single)
    };
    ($first:expr, $second:expr) => {
        concat!("/Game/Maps/", $first, "/", $second)
    };
}

macro_rules! map {
    ($name:expr, $key:expr) => {
        Map {
            display_name: $name.to_owned(),
            image_key: Some($key.to_owned()),
        }
    };
    ($name:expr) => {
        Map {
            display_name: $name.to_owned(),
            image_key: None,
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub display_name: String,
    pub image_key: Option<String>,
}

fn determine_map(match_map: &str) -> Map {
    match match_map {
        map_key!("Ascent") => map!("Ascent", "ascent"),
        map_key!("Bonsai") => map!("Split", "split"),
        map_key!("Canyon") => map!("Fracture", "fracture"),
        map_key!("Duality") => map!("Bind", "bind"),
        map_key!("Foxtrot") => map!("Breeze", "breeze"),
        map_key!("Port") => map!("Icebox", "icebox"),
        map_key!("Poveglia", "Range") => map!("The Range", "range"),
        map_key!("Triad") => map!("Haven", "haven"),
        _ => map!(match_map),
    }
}

fn determine_status(presence: &ParsedPresence) -> GameStateStatus {
    match presence.session_loop_state.as_str() {
        "MENUS" => GameStateStatus::Menu {
            in_queue: presence.party_state == "MATCHMAKING",
        },
        "PREGAME" => GameStateStatus::PreGame,
        "INGAME" => GameStateStatus::InGame,
        _ => panic!("Invalid session loop state."),
    }
}

fn determine_game_mode(presence: &ParsedPresence) -> GameMode {
    if presence.party_state == "CUSTOM_GAME_SETUP" {
        return GameMode::CustomGame;
    }
    match presence.queue_id.as_str() {
        "competitive" => GameMode::Competitive,
        "unrated" => GameMode::Unrated,
        "spikerush" => GameMode::SpikeRush,
        "deathmatch" => GameMode::Deathmatch,
        "onefa" => GameMode::Replication,
        _ => GameMode::Unknown,
    }
}
