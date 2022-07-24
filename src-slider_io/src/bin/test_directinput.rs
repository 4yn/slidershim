extern crate slider_io;

use std::io;

use slider_io::{config::Config, context::Context};

#[tokio::main]
async fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  let config = Config::from_str(
    r##"{
        "deviceMode": "brokenithm",
        "outputMode": "kb-32-tasoller",
        "ledMode": "none",
        "disableAirStrings": false,
        "divaSerialPort": "COM1",
        "divaBrightness": 63,
        "brokenithmPort": 1606,
        "keyboardSensitivity": 20,
        "keyboardDirectInput": true,
        "outputPolling": "100",
        "outputWebsocketUrl": "localhost:3000",
        "ledFaster": false,
        "ledColorActive": "#ff00ff",
        "ledColorInactive": "#ffff00",
        "ledSensitivity": 20,
        "ledWebsocketUrl": "localhost:3001",
        "ledSerialPort": "COM5"
    }"##,
  )
  .unwrap();
  println!("{:?}", config);

  let ctx = Context::new(config);

  println!("Press enter to quit");
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
