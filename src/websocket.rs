use lazy_static::lazy_static;
use reqwest::{Client, Error};

use crate::{lockfile::RiotCredentials, valorant::session::SessionResponse};

pub async fn run_websocket() {}

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
}

pub async fn get_puuid(creds: RiotCredentials) -> Result<String, Error> {
    let response: SessionResponse = CLIENT
        .get(format!("https://127.0.0.1:{}/chat/v1/session", creds.port))
        .basic_auth("riot", Some(creds.password))
        .send()
        .await?
        .json()
        .await?;

    println!("Player UUID: {}\nPlayer Tag: {}#{}", response.puuid, response.game_name, response.game_tag);

    Ok(response.puuid)
}
