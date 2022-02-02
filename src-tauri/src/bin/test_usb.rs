extern crate slidershim;

use std::io;

use slidershim::slider_io::{Config, Manager};

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  // voltex?
  let config = Config::from_str(
    r#"{
        "deviceMode": "yuancon",
        "outputMode": "gamepad-voltex",
        "keyboardSensitivity": 50,
        "ledMode": "reactive-voltex",
        "ledSensitivity": 50
    }"#,
  )
  .unwrap();

  // serial?
  // let config = Config::from_str(
  //   r#"{
  //         "deviceMode": "yuancon",
  //         "outputMode": "kb-32-tasoller",
  //         "keyboardSensitivity": 50,
  //         "ledMode": "serial",
  //         "ledSerialPort": "COM5"
  //     }"#,
  // )
  // .unwrap();

  // basic
  // let config = Config::from_str(
  //   r#"{
  //       "deviceMode": "yuancon",
  //       "outputMode": "kb-32-tasoller",
  //       "keyboardSensitivity": 50,fdwdfp1
  //       "ledMode": "reactive-8",
  //       "ledSensitivity": 50
  //   }"#,
  // )
  // .unwrap();

  // tasoller/
  // let config = Config::from_str(
  //   r#"{
  //       "deviceMode": "tasoller-two",
  //       "outputMode": "kb-32-tasoller",
  //       "keyboardSensitivity": 50,
  //       "ledMode": "reactive-8",
  //       "ledSensitivity": 50
  //   }"#,
  // )
  // .unwrap();

  let manager = Manager::new(config);

  let mut input = String::new();
  let string = io::stdin().read_line(&mut input).unwrap();
}
