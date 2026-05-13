use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, Message},
    Connector,
};
use tracing::{debug, warn};

#[derive(Debug, Clone)]
pub enum LcuEvent {
    ReadyCheck { ready_to_accept: bool },
    GameflowPhase(String),
    QueueId(i64),
}

/// Self-signed cert verifier that accepts any localhost cert. Required for
/// the League Client websocket which is signed by Riot's internal CA.
mod no_verifier {
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
    use rustls::crypto::ring::default_provider;
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
    use rustls::{DigitallySignedStruct, Error, SignatureScheme};
    use std::sync::Arc;

    #[derive(Debug)]
    pub struct AcceptAny;

    impl ServerCertVerifier for AcceptAny {
        fn verify_server_cert(
            &self,
            _: &CertificateDer<'_>,
            _: &[CertificateDer<'_>],
            _: &ServerName<'_>,
            _: &[u8],
            _: UnixTime,
        ) -> Result<ServerCertVerified, Error> {
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _: &[u8],
            _: &CertificateDer<'_>,
            _: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _: &[u8],
            _: &CertificateDer<'_>,
            _: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            default_provider().signature_verification_algorithms.supported_schemes()
        }
    }
}

fn tls_connector() -> Result<Connector> {
    // Ensure the default crypto provider is installed once. ignore the error if
    // it's already installed by a previous call.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(no_verifier::AcceptAny))
        .with_no_client_auth();
    Ok(Connector::Rustls(Arc::new(config)))
}

/// Connect to the LCU websocket and forward decoded events into a channel.
///
/// Returns when the underlying connection drops (or the consumer hangs up),
/// allowing callers to drive reconnect logic externally.
pub async fn run_stream(port: u16, token: &str, tx: mpsc::Sender<LcuEvent>) -> Result<()> {
    let url = format!("wss://127.0.0.1:{}/", port);
    let mut request = url.into_client_request().context("build ws request")?;
    let auth = format!("Basic {}", B64.encode(format!("riot:{}", token)));
    request
        .headers_mut()
        .insert("Authorization", auth.parse().context("auth header")?);

    let connector = tls_connector()?;
    let (mut ws, _) = connect_async_tls_with_config(request, None, false, Some(connector))
        .await
        .context("ws connect")?;

    // Subscribe to all REST events on the bus.
    ws.send(Message::Text(r#"[5,"OnJsonApiEvent"]"#.to_owned()))
        .await
        .context("subscribe OnJsonApiEvent")?;

    debug!(port, "ws connected");

    while let Some(msg) = ws.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(err) => {
                warn!(?err, "ws error");
                break;
            }
        };

        let Message::Text(text) = msg else {
            continue;
        };
        if text.is_empty() {
            continue;
        }

        let Ok(payload) = serde_json::from_str::<serde_json::Value>(&text) else {
            continue;
        };

        let arr = match payload.as_array() {
            Some(a) if a.len() >= 3 && a[0].as_i64() == Some(8) => a,
            _ => continue,
        };

        let event = &arr[2];
        let uri = event.get("uri").and_then(|v| v.as_str()).unwrap_or("");
        let data = event.get("data");

        match uri {
            "/lol-matchmaking/v1/ready-check" => {
                if let Some(d) = data {
                    let state = d.get("state").and_then(|v| v.as_str()).unwrap_or("");
                    let response = d
                        .get("playerResponse")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let ready = state == "InProgress" && response == "None";
                    let _ = tx.send(LcuEvent::ReadyCheck { ready_to_accept: ready }).await;
                }
            }
            "/lol-gameflow/v1/gameflow-phase" => {
                if let Some(phase) = data.and_then(|d| d.as_str()) {
                    let _ = tx.send(LcuEvent::GameflowPhase(phase.to_owned())).await;
                }
            }
            "/lol-gameflow/v1/session" => {
                let queue_id = data
                    .and_then(|d| d.get("gameData"))
                    .and_then(|g| g.get("queue"))
                    .and_then(|q| q.get("id"))
                    .and_then(|i| i.as_i64());
                if let Some(id) = queue_id {
                    let _ = tx.send(LcuEvent::QueueId(id)).await;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
