extern crate slider_io;

use std::io;

use slider_io::config::Config;

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  // voltex?
  let config = Config::from_str(
    r#"{
          "deviceMode": "yuancon",
          "outputMode": "gamepad-voltex",
          "outputPolling": "60",
          "keyboardSensitivity": 50,
          "ledMode": "reactive-voltex",
          "ledSensitivity": 50
      }"#,
  )
  .unwrap();
  println!("{:?}", config);

  // serial?
  let config = Config::from_str(
    r#"{
            "deviceMode": "yuancon",
            "outputMode": "kb-32-tasoller",
            "outputPolling": "60",
            "keyboardSensitivity": 50,
            "ledMode": "serial",
            "ledSerialPort": "COM5"
        }"#,
  )
  .unwrap();
  println!("{:?}", config);

  // basic
  let config = Config::from_str(
    r#"{
          "deviceMode": "yuancon",
          "outputMode": "kb-32-tasoller",
          "keyboardSensitivity": 50,
          "outputPolling": "60",
          "ledMode": "reactive-8",
          "ledSensitivity": 50
      }"#,
  )
  .unwrap();
  println!("{:?}", config);

  // tasoller/
  let config = Config::from_str(
    r#"{
          "deviceMode": "tasoller-two",
          "outputMode": "kb-32-tasoller",
          "outputPolling": "60",
          "keyboardSensitivity": 50,
          "ledMode": "reactive-8",
          "ledSensitivity": 50
      }"#,
  )
  .unwrap();

  println!("{:?}", config);

  // let manager = Context::new(config);

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
