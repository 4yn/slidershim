#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

pub mod config;
pub mod shared;
pub mod state;

pub mod device;
pub mod lighting;
pub mod output;

pub mod system;

pub mod context;
pub mod manager;

pub use config::Config;
pub use manager::Manager;
pub use system::{get_brokenithm_qr_path, get_log_file_path, list_ips};
