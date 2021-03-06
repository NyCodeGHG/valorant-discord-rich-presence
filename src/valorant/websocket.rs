use anyhow::{Error, Result};
use base64::{decode, encode};
use futures::StreamExt;
use futures_util::{stream::SplitSink, SinkExt};
use http::{header::AUTHORIZATION, Request};
use lazy_static::lazy_static;
use native_tls::TlsConnector;
use reqwest::Client;
use tokio::{net::TcpStream, sync::mpsc::Sender};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, handshake::client::Response, Message},
    Connector, MaybeTlsStream, WebSocketStream,
};

use crate::{
    lockfile::RiotCredentials,
    valorant::{presence::PresenceResponse, session::SessionResponse},
};

use super::{
    game_state::GameState,
    presence::{ParsedPresence, Presence},
    presence_analyzer::analyze_presence,
};

pub async fn receive_websocket_events(
    sender: Sender<GameState>,
    creds: RiotCredentials,
) -> Result<()> {
    let own_puuid = get_puuid(&creds).await?;
    let (socket, _) = create_websocket_connection(&creds).await;
    println!("Connected to websocket.");
    let (mut write, read) = futures::StreamExt::split(socket);
    register_ws_event(&mut write, 5, "OnJsonApiEvent_chat_v4_presences").await?;
    println!("Registered for OnJsonApiEvent_chat_v4_presences event.");
    tokio::spawn(async move {
        read.filter_map(|result| async { result.ok() })
            .filter_map(|message| async {
                match message {
                    Message::Text(text) => Some(text),
                    _ => None,
                }
            })
            .filter_map(
                |message| async move { serde_json::from_str::<PresenceResponse>(&message).ok() },
            )
            .map(|response| response.data.data.presences)
            .for_each(|value| async {
                handle_presences(&sender, value, own_puuid.as_str()).await;
            })
            .await;
    });
    Ok(())
}

async fn handle_presences(sender: &Sender<GameState>, presences: Vec<Presence>, own_puuid: &str) {
    let presences: Vec<ParsedPresence> = presences
        .iter()
        .filter(|p| p.product == "valorant" && p.puuid == own_puuid)
        .filter_map(|p| decode(&p.private).ok())
        .filter_map(|json| String::from_utf8(json).ok())
        .filter_map(|json| serde_json::from_str::<ParsedPresence>(&json).ok())
        .collect();
    let presence = match presences.first() {
        Some(p) => p,
        None => return,
    };

    let presence = analyze_presence(presence);
    sender.send(presence).await.unwrap();
}

async fn register_ws_event(
    socket: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    event_number: u32,
    event_name: &str,
) -> Result<(), Error> {
    let message = Message::text(format!("[{},\"{}\"]", event_number, event_name));
    Ok(socket.send(message).await?)
}

async fn create_websocket_connection(
    creds: &RiotCredentials,
) -> (WebSocketStream<MaybeTlsStream<TcpStream>>, Response) {
    let request = build_request(creds);
    connect_async_tls_with_config(request, None, Some(build_ssl_config()))
        .await
        .expect("Failed to connect")
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

pub async fn get_puuid(creds: &RiotCredentials) -> Result<String> {
    let response: SessionResponse = CLIENT
        .get(format!("https://127.0.0.1:{}/chat/v1/session", creds.port))
        .basic_auth("riot", Some(&creds.password))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.puuid)
}
