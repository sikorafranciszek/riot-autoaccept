use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GameflowPhase {
    None,
    Lobby,
    Matchmaking,
    CheckedIntoTournament,
    ReadyCheck,
    ChampSelect,
    GameStart,
    FailedToLaunch,
    InProgress,
    Reconnect,
    WaitingForStats,
    PreEndOfGame,
    EndOfGame,
    TerminatedInError,
    #[serde(other)]
    Unknown,
}

impl GameflowPhase {
    pub fn from_str(value: &str) -> Self {
        match value {
            "None" => Self::None,
            "Lobby" => Self::Lobby,
            "Matchmaking" => Self::Matchmaking,
            "CheckedIntoTournament" => Self::CheckedIntoTournament,
            "ReadyCheck" => Self::ReadyCheck,
            "ChampSelect" => Self::ChampSelect,
            "GameStart" => Self::GameStart,
            "FailedToLaunch" => Self::FailedToLaunch,
            "InProgress" => Self::InProgress,
            "Reconnect" => Self::Reconnect,
            "WaitingForStats" => Self::WaitingForStats,
            "PreEndOfGame" => Self::PreEndOfGame,
            "EndOfGame" => Self::EndOfGame,
            "TerminatedInError" => Self::TerminatedInError,
            _ => Self::Unknown,
        }
    }

    pub fn is_in_queue(self) -> bool {
        matches!(self, Self::Matchmaking)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadyCheckState {
    #[serde(default)]
    pub state: String,
    #[serde(rename = "playerResponse", default)]
    pub player_response: String,
}

impl ReadyCheckState {
    pub fn is_ready_to_accept(&self) -> bool {
        self.state == "InProgress" && self.player_response == "None"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueueType {
    LeagueOfLegends,
    Tft,
    Aram,
    Unknown,
}

impl QueueType {
    /// Map Riot queue id to a coarse category.
    /// References: https://static.developer.riotgames.com/docs/lol/queues.json
    pub fn from_queue_id(id: i64) -> Self {
        match id {
            // TFT queues
            1090 | 1100 | 1110 | 1111 | 1130 | 1150 | 1160 | 1170 | 1180 | 1190 => Self::Tft,
            // ARAM
            450 | 100 => Self::Aram,
            // LoL ranked / normal / clash / etc.
            400 | 420 | 430 | 440 | 480 | 700 | 830 | 840 | 850 | 1020 | 1300 | 1400 => {
                Self::LeagueOfLegends
            }
            _ => Self::Unknown,
        }
    }
}
