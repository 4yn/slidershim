use directories::ProjectDirs;
use std::{convert::TryFrom, fs, path::PathBuf};

use serde_json::Value;

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
  Reactive {
    layout: ReactiveLayout,
    sensitivity: u8,
  },
  Attract,
  Test,
  Websocket {
    url: String,
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
        "kb-6-deemo" => OutputMode::Keyboard {
          layout: KeyboardLayout::Deemo,
          sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
        },
        "websocket" => OutputMode::Websocket {
          url: v["outputWebsocketUrl"].to_string(),
        },
        _ => panic!("Invalid output mode"),
      },
      led_mode: match v["ledMode"].as_str()? {
        "none" => LedMode::None,
        "reactive-4" => LedMode::Reactive {
          layout: ReactiveLayout::Four,
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "reactive-8" => LedMode::Reactive {
          layout: ReactiveLayout::Eight,
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "reactive-16" => LedMode::Reactive {
          layout: ReactiveLayout::Sixteen,
          sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        },
        "attract" => LedMode::Attract,
        "test" => LedMode::Test,
        "websocket" => LedMode::Websocket {
          url: v["ledWebsocketUrl"].to_string(),
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
      "ledWebsocketUrl": "localhost:3001"
    }"#,
    )
    .unwrap()
  }

  fn get_saved_path() -> Option<Box<PathBuf>> {
    let project_dir = ProjectDirs::from("me", "imp.ress", "slidershim").unwrap();
    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir);

    let config_path = config_dir.join("config.json");

    return Some(Box::new(config_path));
  }

  fn load_saved() -> Option<Self> {
    let config_path = Self::get_saved_path()?;
    if !config_path.exists() {
      return None;
    }
    println!("Found saved");
    let mut saved_data = fs::read_to_string(config_path.as_path()).ok()?;
    println!("Loaded saved {}", saved_data);
    return Self::from_str(saved_data.as_str());
  }

  pub fn default() -> Self {
    Self::load_saved()
      .or_else(|| Some(Self::factory()))
      .unwrap()
  }

  pub fn save(&self) -> Option<()> {
    let config_path = Self::get_saved_path()?;
    println!("Saving to {:?}", config_path);
    fs::write(config_path.as_path(), self.raw.as_str()).unwrap();

    Some(())
  }
}
