use serde_json::Value;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum DeviceMode {
  None,
  TasollerOne,
  TasollerTwo,
  Yuancon,
  Brokenithm { ground_only: bool },
}

#[derive(Debug, Clone, Copy)]
pub enum KeyboardLayout {
  Tasoller,
  Yuancon,
  Deemo,
}

#[derive(Debug, Clone)]
pub enum OutputMode {
  None,
  Keyboard {
    layout: KeyboardLayout,
    sensitivity: u8,
  },
  Websocket {
    url: String,
  },
}

#[derive(Debug, Clone, Copy)]
pub enum ReactiveLayout {
  Four,
  Eight,
  Sixteen,
}

#[derive(Debug, Clone)]
pub enum LedMode {
  None,
  Reactive { layout: ReactiveLayout },
  Attract,
  Test,
  Websocket { url: String },
}

#[derive(Debug, Clone)]
pub struct Config {
  pub raw: String,
  pub device_mode: DeviceMode,
  pub output_mode: OutputMode,
  pub led_mode: LedMode,
}

impl Config {
  pub fn from_str(s: &str) -> Config {
    let v: Value = serde_json::from_str(s).unwrap();

    Config {
      raw: s.to_string(),
      device_mode: match v["deviceMode"].as_str().unwrap() {
        "none" => DeviceMode::None,
        "tasoller-one" => DeviceMode::TasollerOne,
        "tasoller-two" => DeviceMode::TasollerTwo,
        "yuancon" => DeviceMode::Yuancon,
        "brokenithm" => DeviceMode::Brokenithm { ground_only: false },
        "brokenithm-ground" => DeviceMode::Brokenithm { ground_only: true },
        _ => panic!("Invalid device mode"),
      },
      output_mode: match v["outputMode"].as_str().unwrap() {
        "none" => OutputMode::None,
        "kb-32-tasoller" => OutputMode::Keyboard {
          layout: KeyboardLayout::Tasoller,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64().unwrap()).unwrap(),
        },
        "kb-32-yuancon" => OutputMode::Keyboard {
          layout: KeyboardLayout::Yuancon,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64().unwrap()).unwrap(),
        },
        "kb-6-deemo" => OutputMode::Keyboard {
          layout: KeyboardLayout::Deemo,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64().unwrap()).unwrap(),
        },
        "websocket" => OutputMode::Websocket {
          url: v["outputWebsocketUrl"].to_string(),
        },
        _ => panic!("Invalid output mode"),
      },
      led_mode: match v["ledMode"].as_str().unwrap() {
        "none" => LedMode::None,
        "reactive-4" => LedMode::Reactive {
          layout: ReactiveLayout::Four,
        },
        "reactive-8" => LedMode::Reactive {
          layout: ReactiveLayout::Eight,
        },
        "reactive-16" => LedMode::Reactive {
          layout: ReactiveLayout::Sixteen,
        },
        "attract" => LedMode::Attract,
        "test" => LedMode::Test,
        "websocket" => LedMode::Websocket {
          url: v["ledWebsocketUrl"].to_string(),
        },
        _ => panic!("Invalid led mode"),
      },
    }
  }
}
