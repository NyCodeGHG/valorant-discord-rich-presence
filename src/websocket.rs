use anyhow::{Error, Result};
use base64::encode;
use futures_util::{SinkExt, StreamExt};
use http::{header::AUTHORIZATION, Request};
use lazy_static::lazy_static;
use native_tls::TlsConnector;
use reqwest::Client;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, Message},
    Connector,
};

use crate::{
    lockfile::RiotCredentials,
    valorant::{presence::PresenceResponse, session::SessionResponse},
};

pub async fn run_websocket(creds: RiotCredentials) -> Result<()> {
    let request = build_request(&creds);
    let (ws_stream, _) = connect_async_tls_with_config(request, None, Some(build_ssl_config()))
        .await
        .expect("Failed to connect");

    let (mut write, read) = ws_stream.split();
    write
        .send(Message::Text(
            "[5,\"OnJsonApiEvent_chat_v4_presences\"]".to_string(),
        ))
        .await
        .expect("Unable to send message");
    read.for_each(|message| async {
        let text = message.unwrap().into_text().unwrap();
        if text.is_empty() {
            return;
        }
        let data: PresenceResponse = serde_json::from_str(text.as_str()).unwrap();
        println!("{:#?}", data);
    })
    .await;
    println!("WebSocket exited.");
    Ok(())
}

fn build_request(creds: &RiotCredentials) -> Request<()> {
    let host = format!("wss://127.0.0.1:{}", creds.port);
    let mut request = host.into_client_request().unwrap();
    let basic_auth = encode(format!("riot:{}", creds.password));
    request.headers_mut().insert(
        AUTHORIZATION,
        format!("Basic {}", basic_auth).parse().unwrap(),
    );
    request
}

fn build_ssl_config() -> Connector {
    Connector::NativeTls(
        TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    )
}

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
}

pub async fn get_puuid(creds: &RiotCredentials) -> Result<String, Error> {
    let response: SessionResponse = CLIENT
        .get(format!("https://127.0.0.1:{}/chat/v1/session", creds.port))
        .basic_auth("riot", Some(&creds.password))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.puuid)
}
