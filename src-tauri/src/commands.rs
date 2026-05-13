use crate::service::AutoAcceptService;
use crate::settings::{self, Settings};
use crate::state::AppState;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub async fn get_state(service: State<'_, AutoAcceptService>) -> Result<AppState, String> {
    Ok(service.get_state().await)
}

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<Settings, String> {
    Ok(settings::load(&app))
}

#[tauri::command]
pub async fn set_auto_accept(
    enabled: bool,
    app: AppHandle,
    service: State<'_, AutoAcceptService>,
) -> Result<AppState, String> {
    let state = service.set_auto_accept(enabled).await;
    let mut s = settings::load(&app);
    s.auto_accept_enabled = enabled;
    settings::save(&app, &s).map_err(|e| e.to_string())?;
    Ok(state)
}

#[tauri::command]
pub async fn update_settings(
    next: Settings,
    app: AppHandle,
    service: State<'_, AutoAcceptService>,
) -> Result<Settings, String> {
    // Apply side effects: autostart, always-on-top, notifications.
    let autostart_mgr = app.autolaunch();
    if next.start_with_windows {
        let _ = autostart_mgr.enable();
    } else {
        let _ = autostart_mgr.disable();
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_always_on_top(next.always_on_top);
    }

    service
        .set_notifications_enabled(next.show_notifications)
        .await;

    settings::save(&app, &next).map_err(|e| e.to_string())?;
    Ok(next)
}

#[tauri::command]
pub async fn reset_stats(
    app: AppHandle,
    service: State<'_, AutoAcceptService>,
) -> Result<AppState, String> {
    let mut s = settings::load(&app);
    s.accepted_count = 0;
    settings::save(&app, &s).map_err(|e| e.to_string())?;
    // Push the reset into the running service.
    let new_state = service.reset_accepted_count().await;
    Ok(new_state)
}

#[tauri::command]
pub async fn show_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: AppHandle) -> Result<(), String> {
    crate::tray::request_quit(&app);
    app.exit(0);
    Ok(())
}

#[tauri::command]
pub async fn system_locale() -> Result<Option<String>, String> {
    Ok(tauri_plugin_os::locale())
}
