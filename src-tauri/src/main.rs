#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

mod slider_io;

use std::sync::{Arc, Mutex};

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
  let config = Arc::new(Mutex::new(Some(slider_io::Config::default())));
  {
    println!("Saving");
    config.lock().unwrap().as_ref().unwrap().save();
  }

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
    .setup(move |app| {
      let app_handle = app.handle();
      let config_clone = Arc::clone(&config);
      app.listen_global("heartbeat", move |e| {
        let config_handle = config_clone.lock().unwrap();
        println!("Heartbeat {}", config_handle.as_ref().unwrap().raw.as_str());
        app_handle
          .emit_all(
            "showConfig",
            Some(config_handle.as_ref().unwrap().raw.as_str().to_string()),
          )
          .unwrap();
      });

      let config_clone = Arc::clone(&config);
      app.listen_global("setConfig", move |event| {
        let payload = event.payload().unwrap();
        println!("Setting config to {}", payload);
        if let Some(new_config) = slider_io::Config::from_str(payload) {
          let mut config_handle = config_clone.lock().unwrap();
          config_handle.replace(new_config);
          config_handle.as_ref().unwrap().save();
        }
      });

      let app_handle = app.handle();
      app.listen_global("hide", move |_| {
        hide_window(&app_handle);
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
