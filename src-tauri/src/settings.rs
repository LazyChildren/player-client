use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{path::BaseDirectory, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppSettings {
    #[serde(alias = "server_ip")]
    server_ip: String,
    #[serde(alias = "server_port")]
    server_port: String,
    #[serde(default, alias = "close_window_behavior")]
    pub(crate) close_window_behavior: Option<String>,
    #[serde(default, alias = "download_directory")]
    pub(crate) download_directory: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_directory: String::new(),
            server_ip: "127.0.0.1".to_string(),
            server_port: "3000".to_string(),
            close_window_behavior: Some("minimize".to_string()),
        }
    }
}

fn settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resolve("setting.json", BaseDirectory::AppConfig)
        .map_err(|error| error.to_string())
}

pub(crate) fn build_base_url(settings: &AppSettings) -> String {
    if settings.server_ip.trim().starts_with("http://")
        || settings.server_ip.trim().starts_with("https://")
    {
        format!(
            "{}:{}",
            settings.server_ip.trim().trim_end_matches('/'),
            settings.server_port.trim()
        )
    } else {
        format!(
            "http://{}:{}",
            settings.server_ip.trim(),
            settings.server_port.trim()
        )
    }
}

#[tauri::command]
pub(crate) fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let path = settings_path(&app)?;

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).map_err(|error| error.to_string()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(AppSettings::default()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub(crate) fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let path = settings_path(&app)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let contents = serde_json::to_string_pretty(&settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

/// Use the native system dialog, with the current download directory selected.
#[tauri::command]
pub(crate) async fn choose_download_directory(
    app: tauri::AppHandle,
    window: tauri::Window,
    initial_path: String,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    tauri::async_runtime::spawn_blocking(move || {
        let mut dialog = app
            .dialog()
            .file()
            .set_title("选择音乐下载的文件夹")
            .set_parent(&window);
        let initial = PathBuf::from(initial_path.trim());
        if initial.is_absolute() && initial.is_dir() {
            dialog = dialog.set_directory(initial);
        } else if let Ok(downloads) = app.path().download_dir() {
            dialog = dialog.set_directory(downloads);
        }
        dialog
            .blocking_pick_folder()
            .map(|folder| {
                folder
                    .into_path()
                    .map(|path| path.to_string_lossy().into_owned())
                    .map_err(|error| error.to_string())
            })
            .transpose()
    })
    .await
    .map_err(|error| error.to_string())?
}
