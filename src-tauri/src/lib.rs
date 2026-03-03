mod config;
mod monitor;
mod notifier;

use config::{Config, ConfigState};
use monitor::MonitorState;
use notifier::NotifierState;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
fn get_system_info(monitor: tauri::State<MonitorState>) -> monitor::SystemInfo {
    monitor.refresh_and_get()
}

#[tauri::command]
fn get_config(state: tauri::State<ConfigState>) -> Config {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn save_config(state: tauri::State<ConfigState>, new_config: Config, app: tauri::AppHandle) {
    let validated = new_config.validated();
    let autostart_changed = {
        let current = state.0.lock().expect("config lock poisoned");
        current.autostart != validated.autostart
    };

    if autostart_changed {
        use tauri_plugin_autostart::ManagerExt;
        let autostart = app.autolaunch();
        if validated.autostart {
            let _ = autostart.enable();
        } else {
            let _ = autostart.disable();
        }
    }

    let mut cfg = state.0.lock().expect("config lock poisoned");
    *cfg = validated;
    cfg.save();
}

#[tauri::command]
fn check_notifications(
    monitor: tauri::State<MonitorState>,
    notifier: tauri::State<NotifierState>,
    config_state: tauri::State<ConfigState>,
    app: tauri::AppHandle,
) -> Vec<String> {
    let info = monitor.refresh_and_get();
    let config = config_state.0.lock().unwrap().clone();
    let notifications = notifier.check(&info, &config);

    let mut sent = Vec::new();
    for n in &notifications {
        let _ = app
            .notification()
            .builder()
            .title(&n.title)
            .body(&n.body)
            .show();
        sent.push(format!("{}: {}", n.title, n.body));
    }
    sent
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .manage(MonitorState::new())
        .manage(NotifierState::new())
        .manage(ConfigState(std::sync::Mutex::new(Config::load())))
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_config,
            save_config,
            check_notifications,
        ])
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("System Monitor")
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
                    "quit" => app.exit(0),
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
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
