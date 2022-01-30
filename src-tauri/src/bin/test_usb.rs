extern crate slidershim;

use std::io;

use slidershim::slider_io::{Config, Manager};

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  // let config = Config::from_str(
  //   r#"{
  //       "deviceMode": "yuancon",
  //       "outputMode": "kb-32-tasoller",
  //       "ledMode": "reactive-8",
  //       "keyboardSensitivity": 50
  //   }"#,
  // )
  // .unwrap();

  let config = Config::from_str(
    r#"{
        "deviceMode": "tasoller-two",
        "outputMode": "kb-32-tasoller",
        "keyboardSensitivity": 50,
        "ledMode": "reactive-8",
        "ledSensitivity": 50
    }"#,
  )
  .unwrap();

  let manager = Manager::new(config);

  let mut input = String::new();
  let string = io::stdin().read_line(&mut input).unwrap();
}
