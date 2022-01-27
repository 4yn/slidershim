#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  CustomMenuItem, Event, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

fn main() {
  tauri::Builder::default()
    .system_tray(
      SystemTray::new().with_menu(
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("slidershim".to_string(), "slidershim").disabled())
          .add_item(CustomMenuItem::new("show".to_string(), "Show"))
          .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
      ),
    )
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        app
            .get_window("main")
            .unwrap()
            .show().ok();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "show" => {
          app
            .get_window("main")
            .unwrap()
            .show().ok();
        }
        "quit" => {
          std::process::exit(0);
        }
        _ => {
          panic!("Unexpected menu item click {}", id.as_str());
        }
      },
      _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app_handle, event| match event {
      Event::CloseRequested { label, api, .. } if label.as_str() == "main" => {
        api.prevent_close();
        app_handle
          .get_window("main")
          .unwrap()
          .hide().ok();
      }
      _ => {}
    });
}
