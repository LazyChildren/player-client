use serde::{Deserialize, Serialize};

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

