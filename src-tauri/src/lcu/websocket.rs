use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use futures_util::{SinkExt, StreamExt};
use native_tls::TlsConnector;
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

fn tls_connector() -> Result<Connector> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .context("build TlsConnector")?;
    Ok(Connector::NativeTls(connector))
}

/// Connect to the LCU websocket and forward decoded events into a channel.
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
