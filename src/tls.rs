use std::{sync::Arc, time::SystemTime};

use async_tungstenite::{
    tokio::{connect_async_with_tls_connector, ConnectStream},
    tungstenite::{client::IntoClientRequest, handshake::client::Response},
    WebSocketStream,
};
use rustls::{
    client::{ServerCertVerified, ServerCertVerifier},
    Certificate, ClientConfig, Error, ServerName,
};

pub struct DangerousCertVerifier;

impl ServerCertVerifier for DangerousCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }
}

/// Connects to a WebSocket server ignoring an invalid SSL Certificate.
///
/// This is needed because Riot uses a self signed local certificate and it's not saved anywhere.
pub async fn connect_async_ignore_certificate<R>(
    request: R,
) -> Result<(WebSocketStream<ConnectStream>, Response), async_tungstenite::tungstenite::Error>
where
    R: IntoClientRequest + Unpin,
{
    let tls_config = Arc::new(
        ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(DangerousCertVerifier))
            .with_no_client_auth(),
    );
    let connector = tls_config.into();
    connect_async_with_tls_connector(request, Some(connector)).await
}
