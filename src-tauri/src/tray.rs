use tauri::Manager;

pub(crate) fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(desktop)]
    {
        use tauri::tray::TrayIconBuilder;
        use tauri::menu::MenuBuilder;
        
        let handle = app.handle();
        
        let menu = MenuBuilder::new(handle)
            .text("show", "打开")
            .text("quit", "退出")
            .build();
        
        match menu {
            Ok(menu) => {
                let default_icon = app.default_window_icon().unwrap().clone();
                let _ = TrayIconBuilder::new()
                    .menu(&menu)
                    .tooltip("simple-music")
                    .icon(default_icon)
                    .show_menu_on_left_click(false)
                    .on_tray_icon_event(|tray, event| {
                        use tauri::tray::TrayIconEvent;
                        use tauri::tray::MouseButton;
                        
                        if let TrayIconEvent::Click {
                            button: mouse_button,
                            ..
                        } = event
                        {
                            match mouse_button {
                                MouseButton::Left => {
                                    let app = tray.app_handle();
                                    if let Some(window) = app.get_webview_window("main") {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                                MouseButton::Right => {
                                    // 右键点击：让系统自动弹出菜单，无需处理
                                }
                                _ => {}
                            }
                        }
                    })
                    .build(handle);
            }
            Err(e) => {
                eprintln!("Failed to create tray menu: {}", e);
            }
        }
    }
    
    Ok(())
}

pub(crate) fn handle_menu_event(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        },
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        },
        _ => {}
    }
}
