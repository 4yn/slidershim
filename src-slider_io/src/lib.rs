#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

mod config;
mod controller_state;
mod shared;

mod device;
mod lighting;
mod output;

mod context;
mod manager;

pub use config::{list_ips, Config};
pub use manager::Manager;
