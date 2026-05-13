use crate::service::{AutoAcceptService, STATE_EVENT};
use crate::state::AppState;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Listener, Manager, Runtime};
use tracing::warn;

static QUIT_REQUESTED: AtomicBool = AtomicBool::new(false);

pub fn request_quit(_app: &AppHandle) {
    QUIT_REQUESTED.store(true, Ordering::SeqCst);
}

pub fn quit_requested() -> bool {
    QUIT_REQUESTED.load(Ordering::SeqCst)
}

pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let toggle = MenuItem::with_id(app, "tray.toggle", "Auto-accept: ON", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "tray.show", "Show", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "tray.hide", "Hide", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "tray.quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle, &show, &hide, &separator, &quit])?;

    let _tray = TrayIconBuilder::with_id("main")
        .tooltip("Riot Auto Accept")
        .icon(
            app.default_window_icon()
                .cloned()
                .ok_or_else(|| tauri::Error::AssetNotFound("default icon".into()))?,
        )
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_window(tray.app_handle());
            }
        })
        .build(app)?;

    // Update tray label when state changes.
    let app_handle = app.clone();
    app.listen(STATE_EVENT, move |event| {
        if let Ok(state) = serde_json::from_str::<AppState>(event.payload()) {
            if let Some(tray) = app_handle.tray_by_id("main") {
                let label = if state.auto_accept_enabled {
                    "Auto-accept: ON"
                } else {
                    "Auto-accept: OFF"
                };
                if let Some(item) = tray
                    .menu()
                    .and_then(|m| m.get("tray.toggle"))
                    .and_then(|i| i.as_menuitem().cloned())
                {
                    let _ = item.set_text(label);
                }
            }
        }
    });

    Ok(())
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id().as_ref() {
        "tray.toggle" => {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                let Some(service) = app.try_state::<AutoAcceptService>() else {
                    return;
                };
                let current = service.get_state().await;
                let _ = service.set_auto_accept(!current.auto_accept_enabled).await;
            });
        }
        "tray.show" => show_window(app),
        "tray.hide" => hide_window(app),
        "tray.quit" => {
            QUIT_REQUESTED.store(true, Ordering::SeqCst);
            app.exit(0);
        }
        _ => warn!(id = ?event.id(), "unhandled tray menu event"),
    }
}

fn toggle_window<R: Runtime>(app: &AppHandle<R>) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
    }
}

fn show_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn hide_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}
