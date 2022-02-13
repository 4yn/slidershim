#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

mod config;
mod shared;
mod state;

mod device;
mod lighting;
mod output;

mod system;

mod context;
mod manager;

pub use config::Config;
pub use manager::Manager;
pub use system::{get_brokenithm_qr_path, get_log_file_path, list_ips};
