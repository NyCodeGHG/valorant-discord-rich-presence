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

    GameState {
        game_mode,
        party,
        scores,
        status,
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
