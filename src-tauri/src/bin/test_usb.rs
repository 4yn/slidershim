extern crate slidershim;

use std::io;

use slidershim::slider_io::{config::Config, manager::Manager};

fn main() {
  let config = Config::from_str(
    r#"{
        "deviceMode": "yuancon",
        "outputMode": "none",
        "ledMode": "reactive-8"
    }"#,
  );

  let manager = Manager::new(config);

  let mut input = String::new();
  let string = io::stdin().read_line(&mut input).unwrap();
}