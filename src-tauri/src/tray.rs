use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Emitter, Manager};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TrayState {
    playing: bool,
    has_tracks: bool,
    volume: u8,
    playback_order: String,
    theme: String,
}

pub(crate) fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage(Mutex::new(TrayState {
        playing: false,
        has_tracks: false,
        volume: 80,
        playback_order: "loop".into(),
        theme: "light".into(),
    }));
    let icon = app.default_window_icon().cloned().ok_or("缺少托盘图标")?;
    tauri::tray::TrayIconBuilder::new()
        .tooltip("simple-music")
        .icon(icon)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button,
                button_state: tauri::tray::MouseButtonState::Up,
                position,
                ..
            } = event
            {
                match button {
                    tauri::tray::MouseButton::Left => show_window(tray.app_handle()),
                    tauri::tray::MouseButton::Right => {
                        if let Err(error) = show_popup(tray.app_handle(), position.x, position.y) {
                            eprintln!("托盘菜单打开失败：{}", error);
                        }
                    }
                    _ => {}
                }
            }
        })
        .build(app.handle())?;
    Ok(())
}

fn show_window(app: &tauri::AppHandle) {
    if let Some(popup) = app.get_webview_window("tray") {
        let _ = popup.hide();
    }
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn show_popup(app: &tauri::AppHandle, x: f64, y: f64) -> tauri::Result<()> {
    let Some(popup) = app.get_webview_window("tray") else {
        return Ok(());
    };
    if popup.is_visible()? {
        popup.hide()?;
        return Ok(());
    }
    let size = popup.outer_size()?;
    let mut left = x as i32 - size.width as i32;
    let mut top = y as i32 - size.height as i32 - 8;
    if let Some(monitor) = popup.monitor_from_point(x, y)? {
        let area = monitor.work_area();
        left = left.clamp(
            area.position.x,
            (area.position.x + area.size.width as i32 - size.width as i32).max(area.position.x),
        );
        top = top.clamp(
            area.position.y,
            (area.position.y + area.size.height as i32 - size.height as i32).max(area.position.y),
        );
    }
    popup.set_position(tauri::PhysicalPosition::new(left, top))?;
    popup.show()?;
    popup.set_focus()?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn resize_tray_menu(app: tauri::AppHandle, height: u32) -> Result<(), String> {
    if let Some(popup) = app.get_webview_window("tray") {
        let old_size = popup.inner_size().map_err(|e| e.to_string())?;
        let position = popup.outer_position().map_err(|e| e.to_string())?;
        popup.set_size(tauri::LogicalSize::new(280.0, height.clamp(200, 700) as f64)).map_err(|e| e.to_string())?;
        if popup.is_visible().map_err(|e| e.to_string())? {
            let new_size = popup.inner_size().map_err(|e| e.to_string())?;
            popup.set_position(tauri::PhysicalPosition::new(position.x, position.y + old_size.height as i32 - new_size.height as i32)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub(crate) async fn sync_tray_state(
    app: tauri::AppHandle,
    playing: bool,
    has_tracks: bool,
    volume: u8,
    playback_order: String,
    theme: String,
) -> Result<(), String> {
    let state = TrayState {
        playing,
        has_tracks,
        volume: volume.min(100),
        playback_order,
        theme,
    };
    *app.state::<Mutex<TrayState>>()
        .lock()
        .map_err(|e| e.to_string())? = state.clone();
    app.emit_to("tray", "tray-state", state)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub(crate) fn get_tray_state(app: tauri::AppHandle) -> Result<TrayState, String> {
    Ok(app
        .state::<Mutex<TrayState>>()
        .lock()
        .map_err(|e| e.to_string())?
        .clone())
}

#[tauri::command]
pub(crate) async fn tray_action(
    app: tauri::AppHandle,
    action: String,
    value: Option<u8>,
) -> Result<(), String> {
    match action.as_str() {
        "show" => show_window(&app),
        "quit" => app.exit(0),
        "hide" => {
            if let Some(popup) = app.get_webview_window("tray") {
                popup.hide().map_err(|e| e.to_string())?;
            }
        }
        "volume" => {
            let volume = value.ok_or("缺少音量参数")?.min(100);
            app.emit_to("main", "tray-control", format!("tray-volume-{}", volume))
                .map_err(|e| e.to_string())?;
        }
        "tray-previous" | "tray-next" | "tray-play-pause" | "tray-mute" | "tray-mode-loop"
        | "tray-mode-one" | "tray-mode-shuffle" => {
            app.emit_to("main", "tray-control", action)
                .map_err(|e| e.to_string())?;
        }
        _ => return Err("未知的托盘操作".into()),
    }
    Ok(())
}
