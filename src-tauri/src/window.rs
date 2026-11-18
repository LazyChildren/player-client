use tauri::Manager;

use crate::settings::load_settings;

#[tauri::command]
pub(crate) fn hide_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

pub(crate) fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    if window.label() == "tray" {
        match event {
            tauri::WindowEvent::Focused(false) => {
                let _ = window.hide();
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        }
        return;
    }
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        let app_handle = window.app_handle();
        if let Ok(settings) = load_settings(app_handle.clone()) {
            let behavior = settings
                .close_window_behavior
                .as_deref()
                .unwrap_or("minimize");
            if behavior == "minimize" {
                api.prevent_close();
                let _ = window.hide();
                return;
            }
        }
        app_handle.exit(0);
    }
}
