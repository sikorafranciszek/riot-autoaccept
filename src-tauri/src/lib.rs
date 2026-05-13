mod commands;
mod lcu;
mod service;
mod settings;
mod state;
mod tray;

use service::AutoAcceptService;
use state::AppState;
use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tracing_subscriber::EnvFilter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("riot_autoaccept_lib=info,warn")),
        )
        .with_target(false)
        .compact()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::get_settings,
            commands::set_auto_accept,
            commands::update_settings,
            commands::reset_stats,
            commands::show_window,
            commands::hide_window,
            commands::quit_app,
            commands::system_locale,
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            // Load persisted settings.
            let persisted = settings::load(&handle);

            // Apply autostart state on boot in case Windows changed it.
            let autostart = handle.autolaunch();
            if persisted.start_with_windows {
                let _ = autostart.enable();
            } else {
                let _ = autostart.disable();
            }

            // Apply always-on-top.
            if let Some(window) = handle.get_webview_window("main") {
                let _ = window.set_always_on_top(persisted.always_on_top);
            }

            // Bootstrap state.
            let initial = AppState {
                auto_accept_enabled: persisted.auto_accept_enabled,
                accepted_count: persisted.accepted_count,
                ..Default::default()
            };

            // Build the service and start its background loop.
            let service =
                AutoAcceptService::new(handle.clone(), initial, persisted.show_notifications);
            app.manage(service.clone());

            tauri::async_runtime::spawn(async move {
                service.start().await;
            });

            // Setup tray icon.
            if let Err(err) = tray::setup(&handle) {
                tracing::warn!(?err, "tray setup failed");
            }

            // Handle "close" → hide-to-tray (when enabled).
            if let Some(window) = handle.get_webview_window("main") {
                let handle_for_close = handle.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        if tray::quit_requested() {
                            return;
                        }
                        let s = settings::load(&handle_for_close);
                        if s.minimize_to_tray {
                            api.prevent_close();
                            if let Some(w) = handle_for_close.get_webview_window("main") {
                                let _ = w.hide();
                            }
                        }
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running riot-autoaccept");
}
