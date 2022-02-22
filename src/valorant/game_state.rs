use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub game_mode: GameMode,
    pub status: GameStateStatus,
    pub scores: Scores,
    pub party: Party,
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
