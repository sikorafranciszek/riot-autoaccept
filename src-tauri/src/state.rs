use crate::lcu::QueueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionStatus {
    Stopped,
    Searching,
    Connecting,
    Connected,
    Disconnected,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatchPhase {
    Idle,
    InQueue,
    ReadyCheck,
    InChampSelect,
    InGame,
    EndOfGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub running: bool,
    pub auto_accept_enabled: bool,
    pub connection_status: ConnectionStatus,
    pub match_phase: MatchPhase,
    pub queue_type: QueueType,
    pub client_port: Option<u16>,
    pub client_source: Option<String>,
    pub queue_duration_sec: u64,
    pub accepted_count: u64,
    pub last_error: Option<String>,
    pub raw_phase: String,
    pub updated_at: i64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running: false,
            auto_accept_enabled: true,
            connection_status: ConnectionStatus::Stopped,
            match_phase: MatchPhase::Idle,
            queue_type: QueueType::Unknown,
            client_port: None,
            client_source: None,
            queue_duration_sec: 0,
            accepted_count: 0,
            last_error: None,
            raw_phase: "None".to_owned(),
            updated_at: chrono::Utc::now().timestamp_millis(),
        }
    }
}
