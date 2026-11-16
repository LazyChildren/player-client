use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf, time::Duration};
use tauri::Manager;

use crate::settings::{build_base_url, load_settings};

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
pub(crate) struct MusicTrack {
    id: i64,
    title: String,
    artist: String,
    cover_url: String,
    src: String,
    local_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QueryAllResponse {
    code: String,
    msg: String,
    data: Vec<MusicRecord>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DeleteResponse {
    code: String,
    msg: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CoverUrlResponse {
    code: String,
    msg: String,
    data: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct UploadResponse {
    code: String,
    msg: String,
    data: Option<String>,
}

#[tauri::command]
pub(crate) async fn delete_music(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let settings = load_settings(app.clone())?;
    let base_url = build_base_url(&settings);
    let url = format!("{}/music/delete", base_url.trim_end_matches('/'));

    let response = reqwest::Client::new()
        .get(&url)
        .query(&[("id", id.to_string())])
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let payload = response
        .json::<DeleteResponse>()
        .await
        .map_err(|error| error.to_string())?;

    if payload.code != "200" {
        return Err(if payload.msg.trim().is_empty() {
            "删除失败".to_string()
        } else {
            payload.msg
        });
    }

    Ok(())
}

#[tauri::command]
pub(crate) async fn fetch_music_list(app: tauri::AppHandle) -> Result<Vec<MusicTrack>, String> {
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
            local_path: existing_download(&app, item.id, &item.url_link)
                .ok()
                .flatten(),
            id: item.id,
            title: item.music_name,
            artist: item.artist,
            cover_url: item.cover_url,
            src: item.url_link,
        })
        .collect())
}

#[tauri::command]
pub(crate) async fn fetch_cover_url(
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
pub(crate) async fn upload_music(
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
        return Err(if payload.msg.trim().is_empty() {
            "失败".to_string()
        } else {
            payload.msg
        });
    }

    Ok(())
}

fn download_path(app: &tauri::AppHandle, id: i64, src: &str) -> Result<PathBuf, String> {
    let settings = load_settings(app.clone())?;
    let directory = if settings.download_directory.trim().is_empty() {
        app.path()
            .download_dir()
            .map_err(|e| e.to_string())?
            .join("Simple Music")
    } else {
        let path = PathBuf::from(settings.download_directory.trim());
        if !path.is_absolute() {
            return Err("下载目录必须是绝对路径".into());
        }
        path
    };
    let url = reqwest::Url::parse(src).map_err(|e| e.to_string())?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err("无效的歌曲下载链接".into());
    }
    let extension = url
        .path()
        .rsplit('.')
        .next()
        .unwrap_or("mp3")
        .to_ascii_lowercase();
    let extension = match extension.as_str() {
        "mp3" | "ogg" | "flac" | "wav" | "m4a" | "aac" | "opus" => extension.as_str(),
        _ => "mp3",
    };
    let key = build_base_url(&settings)
        .bytes()
        .fold(14695981039346656037u64, |hash, b| {
            (hash ^ b as u64).wrapping_mul(1099511628211)
        });
    Ok(directory.join(format!("{:016x}-{}.{}", key, id, extension)))
}

fn existing_download(app: &tauri::AppHandle, id: i64, src: &str) -> Result<Option<String>, String> {
    let path = download_path(app, id, src)?;
    if !path.is_file() || fs::metadata(&path).map_err(|e| e.to_string())?.len() == 0 {
        return Ok(None);
    }
    app.asset_protocol_scope()
        .allow_file(&path)
        .map_err(|e| e.to_string())?;
    Ok(Some(path.to_string_lossy().into_owned()))
}

#[tauri::command]
pub(crate) fn resolve_local_music(
    app: tauri::AppHandle,
    id: i64,
    src: String,
) -> Result<Option<String>, String> {
    existing_download(&app, id, &src)
}

#[tauri::command]
pub(crate) async fn download_music(
    app: tauri::AppHandle,
    id: i64,
    src: String,
) -> Result<String, String> {
    if let Some(path) = existing_download(&app, id, &src)? {
        return Ok(path);
    }
    let path = download_path(&app, id, &src)?;
    fs::create_dir_all(path.parent().ok_or("无效的下载目录")?).map_err(|e| e.to_string())?;
    let mut response = reqwest::Client::builder()
        .timeout(Duration::from_secs(600))
        .build()
        .map_err(|e| e.to_string())?
        .get(&src)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;
    let temp = path.with_extension(format!("{}.part", std::process::id()));
    let result = async {
        let mut file = fs::File::create(&temp).map_err(|e| e.to_string())?;
        let mut size = 0u64;
        while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
            file.write_all(&chunk).map_err(|e| e.to_string())?;
            size += chunk.len() as u64;
        }
        if size == 0 {
            return Err("下载的歌曲文件为空".to_string());
        }
        file.sync_all().map_err(|e| e.to_string())?;
        drop(file);
        fs::rename(&temp, &path).map_err(|e| e.to_string())?;
        app.asset_protocol_scope()
            .allow_file(&path)
            .map_err(|e| e.to_string())?;
        Ok(path.to_string_lossy().into_owned())
    }
    .await;
    if result.is_err() {
        let _ = fs::remove_file(temp);
    }
    result
}
