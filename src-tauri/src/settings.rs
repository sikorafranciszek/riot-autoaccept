use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

const STORE_PATH: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default = "default_true")]
    pub auto_accept_enabled: bool,
    #[serde(default)]
    pub start_with_windows: bool,
    #[serde(default = "default_true")]
    pub minimize_to_tray: bool,
    #[serde(default = "default_true")]
    pub show_notifications: bool,
    #[serde(default)]
    pub always_on_top: bool,
    #[serde(default = "default_language")]
    pub language: String, // "auto" | "pl" | "en"
    #[serde(default)]
    pub accepted_count: u64,
}

fn default_true() -> bool {
    true
}
fn default_language() -> String {
    "auto".to_owned()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_accept_enabled: true,
            start_with_windows: false,
            minimize_to_tray: true,
            show_notifications: true,
            always_on_top: false,
            language: "auto".to_owned(),
            accepted_count: 0,
        }
    }
}

pub fn load<R: Runtime>(app: &AppHandle<R>) -> Settings {
    let store = match app.store(STORE_PATH) {
        Ok(s) => s,
        Err(_) => return Settings::default(),
    };
    let mut settings = Settings::default();

    if let Some(v) = store.get("autoAcceptEnabled") {
        if let Some(b) = v.as_bool() {
            settings.auto_accept_enabled = b;
        }
    }
    if let Some(v) = store.get("startWithWindows") {
        if let Some(b) = v.as_bool() {
            settings.start_with_windows = b;
        }
    }
    if let Some(v) = store.get("minimizeToTray") {
        if let Some(b) = v.as_bool() {
            settings.minimize_to_tray = b;
        }
    }
    if let Some(v) = store.get("showNotifications") {
        if let Some(b) = v.as_bool() {
            settings.show_notifications = b;
        }
    }
    if let Some(v) = store.get("alwaysOnTop") {
        if let Some(b) = v.as_bool() {
            settings.always_on_top = b;
        }
    }
    if let Some(v) = store.get("language") {
        if let Some(s) = v.as_str() {
            settings.language = s.to_owned();
        }
    }
    if let Some(v) = store.get("acceptedCount") {
        if let Some(n) = v.as_u64() {
            settings.accepted_count = n;
        }
    }
    settings
}

pub fn save<R: Runtime>(app: &AppHandle<R>, s: &Settings) -> anyhow::Result<()> {
    let store = app.store(STORE_PATH)?;
    store.set("autoAcceptEnabled", json!(s.auto_accept_enabled));
    store.set("startWithWindows", json!(s.start_with_windows));
    store.set("minimizeToTray", json!(s.minimize_to_tray));
    store.set("showNotifications", json!(s.show_notifications));
    store.set("alwaysOnTop", json!(s.always_on_top));
    store.set("language", json!(s.language));
    store.set("acceptedCount", json!(s.accepted_count));
    store.save()?;
    Ok(())
}

pub fn update_accepted_count<R: Runtime>(app: &AppHandle<R>, count: u64) {
    if let Ok(store) = app.store(STORE_PATH) {
        store.set("acceptedCount", json!(count));
        let _ = store.save();
    }
}
