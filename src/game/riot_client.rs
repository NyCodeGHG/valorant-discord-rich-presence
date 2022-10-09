use std::collections::VecDeque;

use async_tungstenite::{
    tokio::ConnectStream,
    tungstenite::{
        self, client::IntoClientRequest, handshake::client::Request, http::HeaderValue, Message,
    },
    WebSocketStream,
};
use base64::encode;
use futures::{stream::SplitSink, SinkExt, StreamExt};
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot,
};
use tracing::{event, Level};
use url::Url;

use crate::{
    lockfile::{Lockfile, Protocol},
    tls::connect_async_ignore_certificate,
};

mod events;
mod lockfile_retry;

pub use events::*;

use self::lockfile_retry::read_lockfile_with_retry;

type Result<T> = std::result::Result<T, Error>;

/// Handle for a [RiotSocketClient].
///
/// The underlying client will be dropped
/// and the running task will exit once every handle has been dropped.
#[derive(Clone, Debug)]
pub struct RiotSocketClientHandle {
    sender: mpsc::Sender<RiotClientMessage>,
}

pub struct RiotSocketClient {
    confirmation_queue: VecDeque<oneshot::Sender<()>>,
    receiver: Receiver<RiotClientMessage>,
}

type LockfileSupplier = Box<dyn Fn() -> Option<Lockfile> + Send + Sync>;

impl RiotSocketClient {
    pub async fn from_lockfile(
        lockfile_supplier: LockfileSupplier,
    ) -> Result<RiotSocketClientHandle> {
        let (sender, receiver) = mpsc::channel(10);

        let client = RiotSocketClient::new(receiver);

        tokio::spawn(async move { client.run(lockfile_supplier).await });

        Ok(RiotSocketClientHandle { sender })
    }

    fn new(receiver: Receiver<RiotClientMessage>) -> RiotSocketClient {
        RiotSocketClient {
            confirmation_queue: VecDeque::new(),
            receiver,
        }
    }

    async fn run(mut self, lockfile_supplier: LockfileSupplier) {
        loop {
            let lockfile = read_lockfile_with_retry(&lockfile_supplier)
                .await
                .expect("Failed to read lockfile");
            let socket = match connect_async_ignore_certificate(lockfile).await {
                Ok((socket, _)) => socket,
                Err(err) => panic!("{}", err),
            };
            tracing::info!("Successfully connected to websocket");

            let (mut writer, mut reader) = socket.split();
            loop {
                tokio::select! {
                    message = reader.next() => {
                        if let Some(message) = message {
                            match message {
                                Ok(message) => self.handle_websocket_message(message).await,
                                Err(_) => {
                                    tracing::error!("Connection aborted. Trying to reconnect...");
                                    break;
                                },
                            }
                        } else {
                            return;
                        }
                    },
                    message = self.receiver.recv() => {
                        if let Some(message) = message {
                            self.handle_client_message(message, &mut writer).await;
                        } else {
                            return;
                        }
                    }
                }
            }
        }
    }

    async fn handle_websocket_message(&mut self, message: Message) {
        event!(Level::DEBUG, "received websocket message: {}", message);
        match message {
            // confirmation message
            Message::Text(ref value) if value.is_empty() => {
                if let Some(sender) = self.confirmation_queue.pop_front() {
                    sender.send(()).expect("Failed to send confirmation");
                }
            }
            // anything else
            _ => (),
        }
    }

    async fn handle_client_message(
        &mut self,
        message: RiotClientMessage,
        writer: &mut SplitSink<WebSocketStream<ConnectStream>, Message>,
    ) {
        event!(Level::DEBUG, "received client message: {:?}", message);
        match message {
            RiotClientMessage::Subscribe(event, sender) => {
                self.confirmation_queue.push_back(sender);
                writer
                    .send(format!(r#"[5, "{event}"]"#).into())
                    .await
                    .expect("Failed to send message");
            }
            RiotClientMessage::Unsubscribe(event, sender) => {
                self.confirmation_queue.push_back(sender);
                writer
                    .send(format!(r#"[6, "{event}"]"#).into())
                    .await
                    .expect("Failed to send message");
            }
        }
    }
}

impl RiotSocketClientHandle {
    /// Subscribes the client to an event.
    ///
    /// After sending the subscribtion request,
    /// the client waits for a confirmation message from the server.
    pub async fn subscribe(&self, event: Event) {
        let (sender, receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(RiotClientMessage::Subscribe(event, sender))
            .await;
        receiver.await.expect("Actor task has been killed");
    }

    /// Unsubscribes the client from an event.
    ///
    /// After sending the unsubscription request,
    /// the client waits for a confirmation message from the server.
    #[allow(unused)]
    pub async fn unsubscribe(&self, event: Event) {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(RiotClientMessage::Unsubscribe(event, sender))
            .await
            .unwrap();
        receiver.await.expect("Actor task has been killed");
    }
}

#[derive(Debug)]
enum RiotClientMessage {
    Subscribe(Event, oneshot::Sender<()>),
    Unsubscribe(Event, oneshot::Sender<()>),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ConnectionFailed(#[from] tungstenite::Error),
}

const RIOT_WEBSOCKET_USERNAME: &str = "riot";

impl IntoClientRequest for Lockfile {
    fn into_client_request(self) -> tungstenite::Result<Request> {
        let protocol = match self.protocol {
            Protocol::Insecure => "ws",
            Protocol::Secure => "wss",
        };
        let port = self.port;
        let credentials = encode(format!("{RIOT_WEBSOCKET_USERNAME}:{}", self.password));
        let url = Url::parse(&format!("{protocol}://localhost:{port}")).unwrap();
        let mut request = url.into_client_request()?;
        request.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&format!("Basic {credentials}"))?,
        );
        Ok(request)
    }
}
