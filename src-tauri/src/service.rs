use crate::lcu::{self, GameflowPhase, LcuClient, LcuCredentials, QueueType};
use crate::lcu::websocket::LcuEvent;
use crate::state::{AppState, ConnectionStatus, MatchPhase};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, info, warn};

const ACCEPT_DEBOUNCE: Duration = Duration::from_millis(1500);
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(3);
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const QUEUE_TICK_INTERVAL: Duration = Duration::from_secs(1);
const RECONNECT_INITIAL: Duration = Duration::from_secs(1);

pub const STATE_EVENT: &str = "app://state";
pub const ACCEPTED_EVENT: &str = "app://accepted";

#[derive(Clone)]
pub struct AutoAcceptService {
    app: AppHandle,
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    state: AppState,
    worker: Option<JoinHandle<()>>,
    queue_started_at: Option<Instant>,
    last_accept_at: Option<Instant>,
    notifications_enabled: bool,
}

impl AutoAcceptService {
    pub fn new(app: AppHandle, initial: AppState, notifications_enabled: bool) -> Self {
        Self {
            app,
            inner: Arc::new(Mutex::new(Inner {
                state: initial,
                worker: None,
                queue_started_at: None,
                last_accept_at: None,
                notifications_enabled,
            })),
        }
    }

    pub async fn get_state(&self) -> AppState {
        self.inner.lock().await.state.clone()
    }

    pub async fn set_notifications_enabled(&self, enabled: bool) {
        self.inner.lock().await.notifications_enabled = enabled;
    }

    pub async fn set_auto_accept(&self, enabled: bool) -> AppState {
        let mut inner = self.inner.lock().await;
        inner.state.auto_accept_enabled = enabled;
        inner.state.updated_at = chrono::Utc::now().timestamp_millis();
        let state = inner.state.clone();
        drop(inner);
        self.emit_state(&state);
        state
    }

    pub async fn reset_accepted_count(&self) -> AppState {
        let mut inner = self.inner.lock().await;
        inner.state.accepted_count = 0;
        inner.state.updated_at = chrono::Utc::now().timestamp_millis();
        let state = inner.state.clone();
        drop(inner);
        self.emit_state(&state);
        state
    }

    pub async fn start(&self) {
        let mut inner = self.inner.lock().await;
        if inner.state.running {
            return;
        }
        inner.state.running = true;
        inner.state.connection_status = ConnectionStatus::Searching;
        inner.state.last_error = None;
        let state = inner.state.clone();
        drop(inner);
        self.emit_state(&state);

        let app = self.app.clone();
        let inner = self.inner.clone();
        let handle = tokio::spawn(async move {
            run_loop(app, inner).await;
        });
        self.inner.lock().await.worker = Some(handle);
        info!("auto-accept worker started");
    }

    pub async fn stop(&self) {
        let mut inner = self.inner.lock().await;
        if let Some(handle) = inner.worker.take() {
            handle.abort();
        }
        inner.state.running = false;
        inner.state.connection_status = ConnectionStatus::Stopped;
        inner.state.match_phase = MatchPhase::Idle;
        inner.state.queue_type = QueueType::Unknown;
        inner.state.client_port = None;
        inner.state.client_source = None;
        inner.state.queue_duration_sec = 0;
        inner.queue_started_at = None;
        inner.state.raw_phase = "None".to_owned();
        inner.state.updated_at = chrono::Utc::now().timestamp_millis();
        let state = inner.state.clone();
        drop(inner);
        self.emit_state(&state);
        info!("auto-accept worker stopped");
    }

    fn emit_state(&self, state: &AppState) {
        if let Err(err) = self.app.emit(STATE_EVENT, state) {
            warn!(?err, "failed to emit state");
        }
    }
}

async fn run_loop(app: AppHandle, inner: Arc<Mutex<Inner>>) {
    loop {
        // 1. Discovery loop — wait until we find a running League client.
        let creds = loop {
            if let Some(c) = lcu::detect_lcu() {
                break c;
            }
            patch(&app, &inner, |s| {
                s.connection_status = ConnectionStatus::Searching;
                s.client_port = None;
                s.client_source = None;
                s.match_phase = MatchPhase::Idle;
                s.queue_type = QueueType::Unknown;
                s.queue_duration_sec = 0;
                s.raw_phase = "None".to_owned();
            })
            .await;
            sleep(DISCOVERY_INTERVAL).await;
        };

        patch(&app, &inner, |s| {
            s.connection_status = ConnectionStatus::Connecting;
            s.client_port = Some(creds.port);
            s.client_source = Some(creds.source.to_owned());
            s.last_error = None;
        })
        .await;

        // 2. Session loop — websocket + polling until disconnect.
        if let Err(err) = run_session(&app, &inner, creds.clone()).await {
            warn!(?err, "session ended with error");
            patch(&app, &inner, |s| {
                s.connection_status = ConnectionStatus::Disconnected;
                s.client_port = None;
                s.client_source = None;
                s.match_phase = MatchPhase::Idle;
                s.queue_type = QueueType::Unknown;
                s.queue_duration_sec = 0;
                s.raw_phase = "None".to_owned();
                s.last_error = Some(err.to_string());
            })
            .await;
        } else {
            patch(&app, &inner, |s| {
                s.connection_status = ConnectionStatus::Disconnected;
                s.client_port = None;
                s.client_source = None;
                s.match_phase = MatchPhase::Idle;
                s.queue_type = QueueType::Unknown;
                s.queue_duration_sec = 0;
                s.raw_phase = "None".to_owned();
            })
            .await;
        }

        sleep(RECONNECT_INITIAL).await;
    }
}

async fn run_session(
    app: &AppHandle,
    inner: &Arc<Mutex<Inner>>,
    creds: LcuCredentials,
) -> anyhow::Result<()> {
    let client = LcuClient::new(creds.port, &creds.token)?;
    let (tx, mut rx) = mpsc::channel::<LcuEvent>(32);

    // Bootstrap state from REST before WS gives us anything.
    if let Ok(phase) = client.get_text("/lol-gameflow/v1/gameflow-phase").await {
        let _ = tx.send(LcuEvent::GameflowPhase(phase)).await;
    }

    let ws_task = {
        let port = creds.port;
        let token = creds.token.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(err) = lcu::websocket::run_stream(port, &token, tx).await {
                debug!(?err, "ws stream ended");
            }
        })
    };

    patch(app, inner, |s| {
        s.connection_status = ConnectionStatus::Connected;
        s.last_error = None;
    })
    .await;
    info!(port = creds.port, source = creds.source, "connected to LCU");

    // Concurrent polling task as a safety net if WS misses an event.
    let poll_task = {
        let app = app.clone();
        let inner = inner.clone();
        let client = client.clone();
        tokio::spawn(async move {
            loop {
                sleep(POLL_INTERVAL).await;
                if let Ok(ready) = client
                    .get::<lcu::ReadyCheckState>("/lol-matchmaking/v1/ready-check")
                    .await
                {
                    if ready.is_ready_to_accept() {
                        try_accept(&app, &inner, &client).await;
                    }
                }
                if let Ok(phase) = client.get_text("/lol-gameflow/v1/gameflow-phase").await {
                    handle_phase(&app, &inner, &phase).await;
                }
            }
        })
    };

    // Queue timer ticker.
    let tick_task = {
        let app = app.clone();
        let inner = inner.clone();
        tokio::spawn(async move {
            loop {
                sleep(QUEUE_TICK_INTERVAL).await;
                let mut guard = inner.lock().await;
                if let Some(started) = guard.queue_started_at {
                    let secs = started.elapsed().as_secs();
                    if guard.state.queue_duration_sec != secs {
                        guard.state.queue_duration_sec = secs;
                        guard.state.updated_at = chrono::Utc::now().timestamp_millis();
                        let snap = guard.state.clone();
                        drop(guard);
                        let _ = app.emit(STATE_EVENT, &snap);
                    }
                }
            }
        })
    };

    // Main event consumer.
    while let Some(event) = rx.recv().await {
        match event {
            LcuEvent::ReadyCheck { ready_to_accept } => {
                if ready_to_accept {
                    try_accept(app, inner, &client).await;
                }
            }
            LcuEvent::GameflowPhase(phase) => {
                handle_phase(app, inner, &phase).await;
            }
            LcuEvent::QueueId(id) => {
                let qt = QueueType::from_queue_id(id);
                patch(app, inner, |s| s.queue_type = qt).await;
            }
        }
    }

    poll_task.abort();
    tick_task.abort();
    ws_task.abort();
    Ok(())
}

async fn handle_phase(app: &AppHandle, inner: &Arc<Mutex<Inner>>, raw: &str) {
    let phase = GameflowPhase::from_str(raw);
    let is_in_queue = phase.is_in_queue();

    let mut guard = inner.lock().await;
    if is_in_queue && guard.queue_started_at.is_none() {
        guard.queue_started_at = Some(Instant::now());
    }
    if !is_in_queue {
        guard.queue_started_at = None;
        guard.state.queue_duration_sec = 0;
    }

    let match_phase = match phase {
        GameflowPhase::Matchmaking => MatchPhase::InQueue,
        GameflowPhase::ReadyCheck => MatchPhase::ReadyCheck,
        GameflowPhase::ChampSelect => MatchPhase::InChampSelect,
        GameflowPhase::InProgress
        | GameflowPhase::GameStart
        | GameflowPhase::Reconnect
        | GameflowPhase::WaitingForStats => MatchPhase::InGame,
        GameflowPhase::PreEndOfGame | GameflowPhase::EndOfGame => MatchPhase::EndOfGame,
        _ => MatchPhase::Idle,
    };

    guard.state.match_phase = match_phase;
    guard.state.raw_phase = raw.to_owned();
    guard.state.updated_at = chrono::Utc::now().timestamp_millis();
    let snap = guard.state.clone();
    drop(guard);
    let _ = app.emit(STATE_EVENT, &snap);
}

async fn try_accept(app: &AppHandle, inner: &Arc<Mutex<Inner>>, client: &LcuClient) {
    let mut guard = inner.lock().await;
    if !guard.state.auto_accept_enabled {
        return;
    }
    if let Some(last) = guard.last_accept_at {
        if last.elapsed() < ACCEPT_DEBOUNCE {
            return;
        }
    }
    guard.last_accept_at = Some(Instant::now());
    let send_notification = guard.notifications_enabled;
    drop(guard);

    match client
        .post_empty("/lol-matchmaking/v1/ready-check/accept")
        .await
    {
        Ok(status) if status.is_success() || status.as_u16() == 204 => {
            let mut guard = inner.lock().await;
            guard.state.accepted_count = guard.state.accepted_count.saturating_add(1);
            guard.state.updated_at = chrono::Utc::now().timestamp_millis();
            let snap = guard.state.clone();
            drop(guard);
            let _ = app.emit(STATE_EVENT, &snap);
            let _ = app.emit(ACCEPTED_EVENT, &snap.accepted_count);
            crate::settings::update_accepted_count(app, snap.accepted_count);
            info!(total = snap.accepted_count, "ready check accepted");

            if send_notification {
                let _ = app
                    .notification()
                    .builder()
                    .title("Riot Auto Accept")
                    .body("Match accepted")
                    .show();
            }
        }
        Ok(status) => {
            warn!(%status, "accept returned non-success");
        }
        Err(err) => {
            warn!(?err, "accept request failed");
            patch(app, inner, |s| {
                s.last_error = Some(format!("Accept failed: {err}"));
            })
            .await;
        }
    }
}

async fn patch<F>(app: &AppHandle, inner: &Arc<Mutex<Inner>>, f: F)
where
    F: FnOnce(&mut AppState),
{
    let mut guard = inner.lock().await;
    f(&mut guard.state);
    guard.state.updated_at = chrono::Utc::now().timestamp_millis();
    let snap = guard.state.clone();
    drop(guard);
    let _ = app.emit(STATE_EVENT, &snap);
}

#[allow(dead_code)]
pub fn get_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Option<tauri::WebviewWindow<R>> {
    app.get_webview_window("main")
}
