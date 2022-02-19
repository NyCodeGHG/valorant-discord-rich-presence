use serde::Deserialize;

#[derive(Deserialize)]
pub struct SessionResponse {
    pub game_name: String,
    pub game_tag: String,
    pub loaded: bool,
    pub puuid: String,
}
