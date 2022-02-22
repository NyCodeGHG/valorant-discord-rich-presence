use serde::Deserialize;
use serde_tuple::Deserialize_tuple;

#[derive(Deserialize_tuple, Debug)]
pub struct PresenceResponse {
    pub number: u8,
    pub message: String,
    pub data: PresenceData,
}

#[derive(Deserialize, Debug)]
pub struct PresenceData {
    pub data: PresenceDataData,
    #[serde(rename = "eventType")]
    pub event_type: String,
}

#[derive(Deserialize, Debug)]
pub struct PresenceDataData {
    pub presences: Vec<Presence>,
}

#[derive(Deserialize, Debug)]
pub struct Presence {
    pub puuid: String,
    pub product: String,
    pub private: String,
}

#[derive(Deserialize, Debug)]
pub struct ParsedPresence {
    #[serde(rename = "sessionLoopState")]
    pub session_loop_state: String,
    #[serde(rename = "partyOwnerMatchScoreAllyTeam")]
    pub party_owner_match_score_ally_team: u32,
    #[serde(rename = "partyOwnerMatchScoreEnemyTeam")]
    pub party_owner_match_score_enemy_team: u32,
    #[serde(rename = "provisioningFlow")]
    pub provisioning_flo: String,
    #[serde(rename = "matchMap")]
    pub match_map: String,
    #[serde(rename = "partyState")]
    pub party_state: String,
    #[serde(rename = "maxPartySize")]
    pub max_party_size: u32,
    #[serde(rename = "queueId")]
    pub queue_id: String,
    #[serde(rename = "partySize")]
    pub party_size: u32,
}
