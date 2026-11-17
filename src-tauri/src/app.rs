use crate::{music, settings, tray, window};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            settings::load_settings,
            settings::save_settings,
            settings::choose_download_directory,
            music::fetch_music_list,
            music::fetch_cover_url,
            music::upload_music,
            music::delete_music,
            music::download_music,
            music::resolve_local_music,
            window::hide_window
        ])
        .setup(tray::setup)
        .on_menu_event(tray::handle_menu_event)
        .on_window_event(window::handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
