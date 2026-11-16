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

/// Run the native Windows folder picker on a worker so the player stays responsive.
#[tauri::command]
pub(crate) async fn choose_download_directory(
    initial_path: String,
) -> Result<Option<String>, String> {
    #[cfg(target_os = "windows")]
    {
        tauri::async_runtime::spawn_blocking(move || {
            use std::os::windows::process::CommandExt;
            let script = r#"
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding($false)
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Application]::EnableVisualStyles()
$dialog = New-Object System.Windows.Forms.FolderBrowserDialog
$dialog.Description = '选择音乐下载的文件夹'
$dialog.ShowNewFolderButton = $true
$dialog.SelectedPath = $env:SIMPLE_MUSIC_INITIAL_DIRECTORY
$owner = New-Object System.Windows.Forms.Form
$owner.TopMost = $true
$owner.ShowInTaskbar = $false
try {
    if ($dialog.ShowDialog($owner) -eq [System.Windows.Forms.DialogResult]::OK) {
        [Console]::Write($dialog.SelectedPath)
    }
} finally {
    $dialog.Dispose()
    $owner.Dispose()
}
"#;
            // Pass paths as data, never interpolate them into executable PowerShell.
            let output = std::process::Command::new("powershell.exe")
                .args(["-NoProfile", "-STA", "-NonInteractive", "-Command", script])
                .env("SIMPLE_MUSIC_INITIAL_DIRECTORY", initial_path)
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| format!("无法打开文件夹选择器：{}", e))?;
            if !output.status.success() {
                return Err("无法打开 Windows 文件夹选择器".to_string());
            }
            let path = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            if path.is_empty() {
                Ok(None)
            } else {
                Ok(Some(path))
            }
        })
        .await
        .map_err(|e| e.to_string())?
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = initial_path;
        Err("当前平台请手动填写下载目录".into())
    }
}
