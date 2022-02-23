use serde::{Deserialize, Serialize};

use super::presence_analyzer::Map;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub game_mode: GameMode,
    pub status: GameStateStatus,
    pub scores: Scores,
    pub party: Party,
    pub map: Map,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameStateStatus {
    InGame,
    PreGame,
    Menu { in_queue: bool },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scores {
    pub ally_team: u32,
    pub enemy_team: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Party {
    pub size: u32,
    pub max_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameMode {
    Unrated,
    Competitive,
    SpikeRush,
    Deathmatch,
    Replication,
    CustomGame,
    Unknown,
}
impl GameMode {
    pub fn get_display_name(&self) -> String {
        match *self {
            GameMode::Unrated => "Unrated",
            GameMode::Competitive => "Competitive",
            GameMode::SpikeRush => "Spike Rush",
            GameMode::Deathmatch => "Deathmatch",
            GameMode::Replication => "Replication",
            GameMode::CustomGame => "Custom Game",
            GameMode::Unknown => "Unknown",
        }
        .to_owned()
    }
}
