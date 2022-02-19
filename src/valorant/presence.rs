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
