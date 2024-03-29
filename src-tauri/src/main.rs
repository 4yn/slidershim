#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

use slider_io;

use parking_lot::Mutex;
use std::sync::Arc;

use log::info;

use tauri::{
  AppHandle, CustomMenuItem, Manager, RunEvent, Runtime, SystemTray, SystemTrayEvent,
  SystemTrayMenu,
};

fn show_window<R: Runtime>(handle: &AppHandle<R>) {
  handle.emit_all("ackShow", "").ok();
  handle.get_window("main").unwrap().show().ok();
}

fn hide_window<R: Runtime>(handle: &AppHandle<R>) {
  handle.emit_all("ackHide", "").ok();
  handle.get_window("main").unwrap().hide().ok();
}

fn quit_app() {
  std::process::exit(0);
}

fn main() {
  // Setup logger

  #[cfg(debug_assertions)]
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();
  info!("Starting slidershim");

  #[cfg(not(debug_assertions))]
  {
    let log_file_path = slider_io::system::get_log_file_path().unwrap();
    simple_logging::log_to_file(log_file_path.as_path(), log::LevelFilter::Debug).unwrap();
  }

  info!("Loading config");
  let config = Arc::new(Mutex::new(Some(slider_io::Config::load())));
  info!("Loading manager");
  let manager = Arc::new(Mutex::new(slider_io::Manager::new()));
  {
    let config_handle = config.lock();
    let config_handle_ref = config_handle.as_ref().unwrap();
    config_handle_ref.save();
    let manager_handle = manager.lock();
    manager_handle.update_config(config_handle_ref.clone());
  }

  info!("Running tauri");
  tauri::Builder::default()
    .system_tray(
      // System tray content
      SystemTray::new().with_menu(
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("slidershim".to_string(), "slidershim").disabled())
          .add_item(CustomMenuItem::new("show".to_string(), "Show"))
          .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
      ),
    )
    .on_system_tray_event(|app_handle, event| match event {
      // System tray events
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
      // Before app starts

      // Hide event
      let app_handle = app.handle();
      app.listen_global("hide", move |_| {
        hide_window(&app_handle);
      });

      // Quit event
      app.listen_global("quit", |_| {
        quit_app();
      });

      // Show logs
      app.listen_global("openLogfile", |_| {
        let log_file_path = slider_io::get_log_file_path();
        if let Some(log_file_path) = log_file_path {
          open::that(log_file_path.as_path()).ok();
        }
      });

      // Show brokenithm qr
      let config_clone = Arc::clone(&config);
      app.listen_global("openBrokenithmQr", move |_| {
        let config_handle = config_clone.lock();
        let brokenithm_qr_path = slider_io::get_brokenithm_qr_path(
          config_handle
            .as_ref()
            .map(|c| c.device_mode.get_port())
            .unwrap_or(None),
        );
        if let Some(brokenithm_qr_path) = brokenithm_qr_path {
          open::that(brokenithm_qr_path.as_path()).ok();
        }
      });

      // Show repo
      app.listen_global("openRepo", |_| {
        open::that("https://github.com/4yn/slidershim").ok();
      });

      // UI ready event
      let app_handle = app.handle();
      let config_clone = Arc::clone(&config);
      app.listen_global("ready", move |_| {
        let config_handle = config_clone.lock();
        info!("Start signal received");
        app_handle
          .emit_all(
            "showConfig",
            Some(config_handle.as_ref().unwrap().raw.as_str().to_string()),
          )
          .unwrap();

        let ips = slider_io::list_ips();
        if let Ok(ips) = ips {
          app_handle.emit_all("listIps", &ips).unwrap();
        }
      });

      // UI update event
      let app_handle = app.handle();
      let manager_clone = Arc::clone(&manager);
      app.listen_global("queryState", move |_| {
        // app_handle.emit_all("showState", "@@@");
        let (snapshot, timer) = {
          let manager_handle = manager_clone.lock();
          (
            manager_handle.try_get_state().map(|x| x.snapshot()),
            manager_handle.get_timer_state(),
          )
        };
        match snapshot {
          Some(snapshot) => {
            app_handle.emit_all("showState", snapshot).ok();
          }
          _ => {}
        }

        app_handle.emit_all("showTimerState", timer).ok();
      });

      // Config set event
      let config_clone = Arc::clone(&config);
      let manager_clone = Arc::clone(&manager);
      app.listen_global("setConfig", move |event| {
        let payload = event.payload().unwrap();
        info!("Config applied {}", payload);
        if let Some(new_config) = slider_io::Config::from_str(payload) {
          let mut config_handle = config_clone.lock();
          config_handle.take();
          config_handle.replace(new_config);
          let config_handle_ref = config_handle.as_ref().unwrap();
          config_handle_ref.save();
          let manager_handle = manager_clone.lock();
          manager_handle.update_config(config_handle_ref.clone());
        }
      });

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app_handle, event| match event {
      // After app starts
      RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
        hide_window(app_handle);
      }
      _ => {}
    });
}
