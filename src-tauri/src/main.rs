#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

mod slider_io;

use tauri::{
  AppHandle, CustomMenuItem, Event, Manager, Runtime, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

fn show_window<R: Runtime>(handle: &AppHandle<R>) {
  handle.get_window("main").unwrap().show().ok();
}

fn hide_window<R: Runtime>(handle: &AppHandle<R>) {
  handle.get_window("main").unwrap().hide().ok();
}

fn quit_app() {
  std::process::exit(0);
}

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
    .on_system_tray_event(|app_handle, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        show_window(app_handle);
      }
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "show" => {
          show_window(app_handle);
        }
        "quit" => {
          quit_app();
        }
        _ => {
          panic!("Unexpected menu item click {}", id.as_str());
        }
      },
      _ => {}
    })
    .setup(|app| {
      app.listen_global("setConfig", |event| {
        let payload = event.payload().unwrap();
        println!("Setting config to {}", payload);
        
      });

      let handle = app.handle();
      app.listen_global("hide", move |_| {
        hide_window(&handle);
      });

      app.listen_global("quit", |_| {
        quit_app();
      });

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app_handle, event| match event {
      Event::CloseRequested { label, api, .. } if label.as_str() == "main" => {
        api.prevent_close();
        hide_window(app_handle);
      }
      _ => {}
    });
}
