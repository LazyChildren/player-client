use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::path::BaseDirectory;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppSettings {
    #[serde(alias = "server_ip")]
    server_ip: String,
    #[serde(alias = "server_port")]
    server_port: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            server_ip: "127.0.0.1".to_string(),
            server_port: "3000".to_string(),
        }
    }
}

fn settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resolve("setting.json", BaseDirectory::AppConfig)
        .map_err(|error| error.to_string())
}

fn build_base_url(settings: &AppSettings) -> String {
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
fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let path = settings_path(&app)?;

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).map_err(|error| error.to_string()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(AppSettings::default()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let path = settings_path(&app)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let contents = serde_json::to_string_pretty(&settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MusicRecord {
    id: i64,
    music_name: String,
    artist: String,
    cover_url: String,
    url_link: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MusicTrack {
    id: i64,
    title: String,
    artist: String,
    cover_url: String,
    src: String,
}

#[derive(Debug, Deserialize)]
struct QueryAllResponse {
    code: String,
    msg: String,
    data: Vec<MusicRecord>,
}

#[derive(Debug, Deserialize)]
struct CoverUrlResponse {
    code: String,
    msg: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct UploadResponse {
    code: String,
    msg: String,
    data: String,
}

#[tauri::command]
async fn fetch_music_list(app: tauri::AppHandle) -> Result<Vec<MusicTrack>, String> {
    let settings = load_settings(app.clone())?;
    let base_url = build_base_url(&settings);
    let url = format!("{}/music/queryAll", base_url.trim_end_matches('/'));

    let response = reqwest::get(&url)
        .await
        .map_err(|error| error.to_string())?;
    let payload = response
        .json::<QueryAllResponse>()
        .await
        .map_err(|error| error.to_string())?;

    if payload.code != "200" {
        return Err(format!("{}: {}", payload.code, payload.msg));
    }

    Ok(payload
        .data
        .into_iter()
        .map(|item| MusicTrack {
            id: item.id,
            title: item.music_name,
            artist: item.artist,
            cover_url: item.cover_url,
            src: item.url_link,
        })
        .collect())
}

#[tauri::command]
async fn fetch_cover_url(
    app: tauri::AppHandle,
    music_name: Option<String>,
    artist: Option<String>,
) -> Result<String, String> {
    let music_name = music_name.unwrap_or_default().trim().to_string();
    let artist = artist.unwrap_or_default().trim().to_string();

    if music_name.is_empty() && artist.is_empty() {
        return Err("歌曲名和艺术家不能同时为空".to_string());
    }

    let settings = load_settings(app.clone())?;
    let base_url = build_base_url(&settings);
    let url = format!("{}/music/getCoverUrl", base_url.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&[
            ("musicName", music_name.as_str()),
            ("artist", artist.as_str()),
        ])
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let payload = response
        .json::<CoverUrlResponse>()
        .await
        .map_err(|error| error.to_string())?;

    if payload.code != "200" {
        return Err("出错请稍后再试".to_string());
    }

    Ok(payload.data)
}

#[tauri::command]
async fn upload_music(
    app: tauri::AppHandle,
    file_name: String,
    file_bytes: Vec<u8>,
    music_name: String,
    artist: String,
    cover_url: String,
) -> Result<(), String> {
    let settings = load_settings(app.clone())?;
    let base_url = build_base_url(&settings);
    let url = format!("{}/music/upload", base_url.trim_end_matches('/'));

    let part = reqwest::multipart::Part::bytes(file_bytes).file_name(file_name);
    let form = reqwest::multipart::Form::new()
        .part("file", part)
        .text("musicName", music_name)
        .text("artist", artist)
        .text("coverUrl", cover_url);

    let response = reqwest::Client::new()
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let payload = response
        .json::<UploadResponse>()
        .await
        .map_err(|error| error.to_string())?;

    if payload.code != "200" {
        return Err("失败".to_string());
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            fetch_music_list,
            fetch_cover_url,
            upload_music
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
