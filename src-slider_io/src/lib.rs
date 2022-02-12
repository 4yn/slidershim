#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(div_duration)]
#![feature(more_qualified_paths)]

mod slider_io;

pub use slider_io::list_ips;
pub use slider_io::Config;
pub use slider_io::Manager;
