use directories::ProjectDirs;
use log::info;
use serde_json::Value;
use std::{convert::TryFrom, fs, path::PathBuf};

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
  Voltex,
  GamepadVoltex,
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
  Even { splits: usize },
  Voltex,
}

#[derive(Debug, Clone)]
pub enum LedMode {
  None,
  Reactive {
    layout: ReactiveLayout,
    sensitivity: u8,
  },
  Attract,
  Test,
  Websocket {
    url: String,
  },
  Serial {
    port: String,
  },
}

#[derive(Debug, Clone)]
pub struct Config {
  pub raw: String,
  pub device_mode: DeviceMode,
  pub output_mode: OutputMode,
  pub led_mode: LedMode,
}

impl Config {
  pub fn from_str(s: &str) -> Option<Config> {
    let v: Value = serde_json::from_str(s).ok()?;

    Some(Config {
      raw: s.to_string(),
      device_mode: match v["deviceMode"].as_str()? {
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
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
        },
        "kb-32-yuancon" => OutputMode::Keyboard {
          layout: KeyboardLayout::Yuancon,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
        },
        "kb-8-deemo" => OutputMode::Keyboard {
          layout: KeyboardLayout::Deemo,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
        },
        "kb-voltex" => OutputMode::Keyboard {
          layout: KeyboardLayout::Voltex,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
        },
        "gamepad-voltex" => OutputMode::Keyboard {
          layout: KeyboardLayout::GamepadVoltex,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
        },
        "websocket" => OutputMode::Websocket {
          url: v["outputWebsocketUrl"].as_str()?.to_string(),
        },
        _ => panic!("Invalid output mode"),
      },
      led_mode: match v["ledMode"].as_str()? {
        "none" => LedMode::None,
        "reactive-4" => LedMode::Reactive {
          layout: ReactiveLayout::Even { splits: 4 },
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "reactive-8" => LedMode::Reactive {
          layout: ReactiveLayout::Even { splits: 8 },
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "reactive-16" => LedMode::Reactive {
          layout: ReactiveLayout::Even { splits: 16 },
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "reactive-voltex" => LedMode::Reactive {
          layout: ReactiveLayout::Voltex,
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "attract" => LedMode::Attract,
        "test" => LedMode::Test,
        "websocket" => LedMode::Websocket {
          url: v["ledWebsocketUrl"].as_str()?.to_string(),
        },
        "serial" => LedMode::Serial {
          port: v["ledSerialPort"].as_str()?.to_string(),
        },
        _ => panic!("Invalid led mode"),
      },
    })
  }

  fn factory() -> Self {
    Self::from_str(
      r#"{
      "deviceMode": "none",
      "outputMode": "none",
      "ledMode": "none",
      "keyboardSensitivity": 20,
      "outputWebsocketUrl": "localhost:3000",
      "ledSensitivity": 20,
      "ledWebsocketUrl": "localhost:3001",
      "ledSerialPort": "COM5"
    }"#,
    )
    .unwrap()
  }

  fn get_saved_path() -> Option<Box<PathBuf>> {
    let project_dir = ProjectDirs::from("me", "imp.ress", "slidershim").unwrap();
    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir).unwrap();

    let config_path = config_dir.join("config.json");

    return Some(Box::new(config_path));
  }

  fn load_saved() -> Option<Self> {
    let config_path = Self::get_saved_path()?;
    if !config_path.exists() {
      return None;
    }
    info!("Config file found at {:?}", config_path);
    let saved_data = fs::read_to_string(config_path.as_path()).ok()?;
    return Self::from_str(saved_data.as_str());
  }

  pub fn default() -> Self {
    Self::load_saved()
      .or_else(|| Some(Self::factory()))
      .unwrap()
  }

  pub fn save(&self) -> Option<()> {
    info!("Config saving...");
    let config_path = Self::get_saved_path()?;
    info!("Config saving to {:?}", config_path);
    fs::write(config_path.as_path(), self.raw.as_str()).unwrap();

    info!("Config saved");
    Some(())
  }
}
