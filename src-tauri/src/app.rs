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
            music::fetch_collections,
            music::fetch_cover_url,
            music::upload_music,
            music::delete_music,
            music::download_music,
            music::resolve_local_music,
            tray::sync_tray_state,
            tray::get_tray_state,
            tray::tray_action,
            tray::resize_tray_menu,
            window::hide_window
        ])
        .setup(tray::setup)
        .on_window_event(window::handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
