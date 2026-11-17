use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::Write, path::{Path, PathBuf}, sync::{Mutex, atomic::{AtomicU64, Ordering}}, time::Duration};
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Collection {
    id: i64,
    collection_name: String,
    created_time: String,
    modified_time: String,
    valid: i32,
}

#[derive(Debug, Deserialize)]
struct CollectionResponse {
    code: String,
    msg: String,
    data: Vec<Collection>,
}

#[tauri::command]
pub(crate) async fn fetch_collections(app: tauri::AppHandle) -> Result<Vec<Collection>, String> {
    let settings = load_settings(app)?;
    let url = format!("{}/collection/queryAll", build_base_url(&settings).trim_end_matches('/'));
    let payload = reqwest::get(&url)
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?
        .json::<CollectionResponse>()
        .await
        .map_err(|error| error.to_string())?;
    if payload.code != "200" {
        return Err(format!("{}: {}", payload.code, payload.msg));
    }
    Ok(payload.data)
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
pub(crate) async fn delete_music(app: tauri::AppHandle, id: i64, src: String) -> Result<Option<String>, String> {
    let settings = load_settings(app.clone())?;
    let base_url = build_base_url(&settings);
    let url = format!("{}/music/delete", base_url.trim_end_matches('/'));
    let legacy = legacy_download_path(&app, id, &src)?;
    let key = download_key(&app, id)?;

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

    match remove_download(legacy.parent().ok_or("无效的下载目录")?, &key, &legacy) {
        Ok(()) => Ok(None),
        Err(error) => Ok(Some(format!("歌曲已从服务器删除，但本地清理失败：{}", error))),
    }
}

#[tauri::command]
pub(crate) async fn fetch_music_list(app: tauri::AppHandle, collection_id: i64) -> Result<Vec<MusicTrack>, String> {
    let settings = load_settings(app.clone())?;
    let base_url = build_base_url(&settings);
    let url = format!("{}/music/queryByCollection", base_url.trim_end_matches('/'));

    let response = reqwest::Client::new()
        .get(&url)
        .query(&[("collectionId", collection_id)])
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
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
            local_path: existing_download(&app, item.id, &item.url_link, &item.music_name, &item.artist)
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
    collection_id: i64,
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
        .text("collectionId", collection_id.to_string())
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

fn legacy_download_path(app: &tauri::AppHandle, id: i64, src: &str) -> Result<PathBuf, String> {
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

fn filename_part(value: &str, fallback: &str) -> String {
    let cleaned: String = value.trim().chars().take(80).map(|c| {
        if c.is_control() || "<>:\"/\\|?*".contains(c) { '_' } else { c }
    }).collect();
    let cleaned = cleaned.trim().trim_end_matches(['.', ' ']);
    if cleaned.is_empty() { fallback.to_string() } else { cleaned.to_string() }
}

fn download_path(app: &tauri::AppHandle, id: i64, src: &str, title: &str, artist: &str) -> Result<PathBuf, String> {
    let legacy = legacy_download_path(app, id, src)?;
    let extension = legacy.extension().and_then(|value| value.to_str()).unwrap_or("mp3");
    Ok(legacy.with_file_name(format!("{}-{}.{}", filename_part(title, "未知歌曲"), filename_part(artist, "未知艺术家"), extension)))
}

static DOWNLOAD_INDEX_LOCK: Mutex<()> = Mutex::new(());
static DOWNLOAD_TEMP_ID: AtomicU64 = AtomicU64::new(0);
const DOWNLOAD_INDEX: &str = ".simple-music-downloads.json";

fn write_download_index(directory: &Path, index: &BTreeMap<String, String>) -> Result<(), String> {
    let index_temp = directory.join(format!("{}.{}.tmp", DOWNLOAD_INDEX, std::process::id()));
    let bytes = serde_json::to_vec_pretty(index).map_err(|e| e.to_string())?;
    let mut file = fs::File::create(&index_temp).map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    fs::rename(&index_temp, directory.join(DOWNLOAD_INDEX)).map_err(|e| e.to_string())
}

fn remove_download(directory: &Path, key: &str, legacy: &Path) -> Result<(), String> {
    let _guard = DOWNLOAD_INDEX_LOCK.lock().map_err(|e| e.to_string())?;
    let mut index = read_download_index(directory)?;
    if let Some(name) = index.get(key) {
        if Path::new(name).components().count() != 1 || Path::new(name).file_name().and_then(|s| s.to_str()) != Some(name.as_str()) {
            return Err("下载记录中的文件名无效".into());
        }
        let path = directory.join(name);
        match fs::remove_file(&path) {
            Ok(()) => (),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error.to_string()),
        }
    }
    // Legacy filenames also identify the server and song ID unambiguously.
    match fs::remove_file(legacy) {
        Ok(()) => (),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
        Err(error) => return Err(error.to_string()),
    }
    if index.remove(key).is_some() {
        write_download_index(directory, &index)?;
    }
    Ok(())
}

fn read_download_index(directory: &Path) -> Result<BTreeMap<String, String>, String> {
    match fs::read(directory.join(DOWNLOAD_INDEX)) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| format!("下载记录读取失败：{}", e)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(error) => Err(error.to_string()),
    }
}

fn download_key(app: &tauri::AppHandle, id: i64) -> Result<String, String> {
    Ok(format!("{}|{}", build_base_url(&load_settings(app.clone())?).trim_end_matches('/'), id))
}

fn indexed_download(directory: &Path, key: &str) -> Result<Option<PathBuf>, String> {
    let index = read_download_index(directory)?;
    let Some(name) = index.get(key) else { return Ok(None) };
    // Records may only point to a filename inside this download directory.
    if Path::new(name).components().count() != 1 || Path::new(name).file_name().and_then(|s| s.to_str()) != Some(name.as_str()) {
        return Err("下载记录中的文件名无效".into());
    }
    let path = directory.join(name);
    Ok(if path.is_file() && fs::metadata(&path).map_err(|e| e.to_string())?.len() > 0 { Some(path) } else { None })
}

fn save_download(directory: &Path, key: &str, desired: &Path, temp: &Path) -> Result<PathBuf, String> {
    let _guard = DOWNLOAD_INDEX_LOCK.lock().map_err(|e| e.to_string())?;
    if let Some(path) = indexed_download(directory, key)? { return Ok(path); }
    let mut index = read_download_index(directory)?;
    let stem = desired.file_stem().and_then(|s| s.to_str()).ok_or("无效的歌曲文件名")?;
    let extension = desired.extension().and_then(|s| s.to_str()).unwrap_or("mp3");
    let mut number = 1u64;
    let (path, mut destination) = loop {
        let path = if number == 1 { desired.to_path_buf() } else { directory.join(format!("{}-{}.{}", stem, number, extension)) };
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => break (path, file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => number += 1,
            Err(error) => return Err(error.to_string()),
        }
    };
    let result = (|| {
        let mut source = fs::File::open(temp).map_err(|e| e.to_string())?;
        std::io::copy(&mut source, &mut destination).map_err(|e| e.to_string())?;
        destination.sync_all().map_err(|e| e.to_string())?;
        index.insert(key.to_string(), path.file_name().unwrap().to_string_lossy().into_owned());
        write_download_index(directory, &index)?;
        Ok::<(), String>(())
    })();
    drop(destination);
    if let Err(error) = result { let _ = fs::remove_file(&path); return Err(error); }
    Ok(path)
}

fn existing_download(app: &tauri::AppHandle, id: i64, src: &str, title: &str, artist: &str) -> Result<Option<String>, String> {
    let desired = download_path(app, id, src, title, artist)?;
    let key = download_key(app, id)?;
    let _guard = DOWNLOAD_INDEX_LOCK.lock().map_err(|e| e.to_string())?;
    let path = indexed_download(desired.parent().ok_or("无效的下载目录")?, &key)?
        .unwrap_or(legacy_download_path(app, id, src)?);
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
    title: String,
    artist: String,
) -> Result<Option<String>, String> {
    existing_download(&app, id, &src, &title, &artist)
}

#[tauri::command]
pub(crate) async fn download_music(
    app: tauri::AppHandle,
    id: i64,
    src: String,
    title: String,
    artist: String,
) -> Result<String, String> {
    if let Some(path) = existing_download(&app, id, &src, &title, &artist)? {
        return Ok(path);
    }
    let path = download_path(&app, id, &src, &title, &artist)?;
    let key = download_key(&app, id)?;
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
    let temp = path.parent().ok_or("无效的下载目录")?.join(format!(".simple-music-{}-{}.part", std::process::id(), DOWNLOAD_TEMP_ID.fetch_add(1, Ordering::Relaxed)));
    let result = async {
        let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&temp).map_err(|e| e.to_string())?;
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
        let path = save_download(path.parent().ok_or("无效的下载目录")?, &key, &path, &temp)?;
        app.asset_protocol_scope()
            .allow_file(&path)
            .map_err(|e| e.to_string())?;
        Ok(path.to_string_lossy().into_owned())
    }
    .await;
    let _ = fs::remove_file(temp);
    result
}

#[cfg(test)]
mod download_tests {
    use super::*;

    struct TestDirectory(PathBuf);
    impl TestDirectory {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("simple-music-test-{}-{}", std::process::id(), DOWNLOAD_TEMP_ID.fetch_add(1, Ordering::Relaxed)));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for TestDirectory {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }

    #[test]
    fn same_names_keep_separate_tracks_and_reuse_their_own_files() {
        let directory = TestDirectory::new();
        let desired = directory.0.join("歌曲-艺术家.mp3");
        let first_temp = directory.0.join("first.part");
        let second_temp = directory.0.join("second.part");
        fs::write(&first_temp, b"first recording").unwrap();
        fs::write(&second_temp, b"second recording").unwrap();
        let first = save_download(&directory.0, "http://server:3000|1", &desired, &first_temp).unwrap();
        let second = save_download(&directory.0, "http://server:3000|2", &desired, &second_temp).unwrap();
        assert_eq!(first.file_name().unwrap(), "歌曲-艺术家.mp3");
        assert_eq!(second.file_name().unwrap(), "歌曲-艺术家-2.mp3");
        assert_eq!(fs::read(&first).unwrap(), b"first recording");
        assert_eq!(fs::read(&second).unwrap(), b"second recording");
        assert_eq!(indexed_download(&directory.0, "http://server:3000|2").unwrap(), Some(second.clone()));
        assert_eq!(save_download(&directory.0, "http://server:3000|2", &desired, &first_temp).unwrap(), second);
        assert_eq!(indexed_download(&directory.0, "http://other-server:3000|1").unwrap(), None);
        fs::remove_file(&first).unwrap();
        assert_eq!(indexed_download(&directory.0, "http://server:3000|1").unwrap(), None);
    }

    #[test]
    fn untracked_files_are_never_claimed_or_overwritten() {
        let directory = TestDirectory::new();
        let desired = directory.0.join("歌曲-艺术家.ogg");
        fs::write(&desired, b"untracked audio").unwrap();
        let temp = directory.0.join("new.part");
        fs::write(&temp, b"new audio").unwrap();
        assert_eq!(indexed_download(&directory.0, "server|1").unwrap(), None);
        let saved = save_download(&directory.0, "server|1", &desired, &temp).unwrap();
        assert_eq!(saved.file_name().unwrap(), "歌曲-艺术家-2.ogg");
        assert_eq!(fs::read(&desired).unwrap(), b"untracked audio");
    }

    #[test]
    fn deletion_removes_only_the_selected_song_and_its_mapping() {
        let directory = TestDirectory::new();
        let desired = directory.0.join("歌曲-艺术家.mp3");
        let temp = directory.0.join("audio.part");
        fs::write(&temp, b"audio").unwrap();
        let first = save_download(&directory.0, "server|1", &desired, &temp).unwrap();
        let second = save_download(&directory.0, "server|2", &desired, &temp).unwrap();
        let legacy = directory.0.join("legacy-1.mp3");
        fs::write(&legacy, b"old audio").unwrap();
        remove_download(&directory.0, "server|1", &legacy).unwrap();
        assert!(!first.exists());
        assert!(!legacy.exists());
        assert!(second.exists());
        let index = read_download_index(&directory.0).unwrap();
        assert!(!index.contains_key("server|1"));
        assert!(index.contains_key("server|2"));
        // Missing local files must not prevent removing stale mappings.
        fs::remove_file(&second).unwrap();
        remove_download(&directory.0, "server|2", &legacy).unwrap();
        assert!(read_download_index(&directory.0).unwrap().is_empty());
        remove_download(&directory.0, "server|2", &legacy).unwrap();
    }

    #[test]
    fn deletion_rejects_paths_outside_download_directory() {
        let directory = TestDirectory::new();
        let mut index = BTreeMap::new();
        index.insert("server|1".into(), "../outside.mp3".into());
        write_download_index(&directory.0, &index).unwrap();
        assert!(remove_download(&directory.0, "server|1", &directory.0.join("legacy.mp3")).is_err());
        assert!(read_download_index(&directory.0).unwrap().contains_key("server|1"));
    }
}
