pub mod client;
pub mod discovery;
pub mod types;
pub mod websocket;

pub use client::LcuClient;
pub use discovery::{detect_lcu, LcuCredentials};
pub use types::{GameflowPhase, QueueType, ReadyCheckState};
