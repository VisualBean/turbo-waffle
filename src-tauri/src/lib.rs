pub mod commands;
pub mod models;
pub mod services;
pub mod storage;

use commands::{
    check_all_health, check_health, delete_connection, get_connections, lookup_mac,
    open_connection, reorder_connections, save_connection, send_wol,
};
use storage::ConfigStorage;
use tauri::menu::MenuBuilder;
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, Runtime, WindowEvent};

fn build_tray_menu<R: Runtime, M: Manager<R>>(manager: &M) -> tauri::Result<tauri::menu::Menu<R>> {
    let mut connections = ConfigStorage::new()
        .and_then(|storage| storage.load_connections())
        .unwrap_or_default();

    connections.sort_by(|a, b| {
        a.order
            .cmp(&b.order)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    let mut builder = MenuBuilder::new(manager);

    if connections.is_empty() {
        builder = builder.text("connections_empty", "No connections");
    } else {
        for connection in connections {
            let label = connection.name;
            let id = format!("connect:{}", connection.id);
            builder = builder.text(id, label);
        }
    }

    builder
        .separator()
        .text("show_main", "Open Turbo Waffle")
        .separator()
        .text("exit", "Exit")
        .build()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let menu = build_tray_menu(app.handle())?;
            let icon = tauri::include_image!("icons/tray-32.png");

            TrayIconBuilder::with_id("main_tray")
                .icon(icon)
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .build(app)?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id() == "exit" {
                app.exit(0);
                return;
            }

            if event.id() == "show_main" {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                return;
            }

            if let Some(id) = event.id().0.strip_prefix("connect:") {
                let id = id.to_string();
                let handle = app.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    let _ = open_connection(handle, id).await;
                });
            }
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_connections,
            save_connection,
            delete_connection,
            reorder_connections,
            check_health,
            check_all_health,
            send_wol,
            lookup_mac,
            open_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
