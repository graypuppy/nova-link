use crate::config;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

/// 创建系统托盘
pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&show_item, &quit_item]).unwrap();

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                // 保存窗口状态后退出
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(pos) = window.outer_position() {
                        if let Ok(size) = window.outer_size() {
                            let _ = config::save_window_state(
                                pos.x,
                                pos.y,
                                size.width,
                                size.height,
                            );
                        }
                    }
                }
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
