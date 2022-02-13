use log::{info, warn};
use serde_json::Value;
use std::fs;

use crate::{
  input::config::DeviceMode, lighting::config::LedMode, output::config::OutputMode, system,
};

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
      device_mode: DeviceMode::from_serde_value(&v)?,
      output_mode: OutputMode::from_serde_value(&v)?,
      led_mode: LedMode::from_serde_value(&v)?,
    })
  }

  fn default() -> Self {
    Self::from_str(
      r#"{
      "deviceMode": "none",
      "devicePolling": "100",
      "outputMode": "none",
      "ledMode": "none",
      "keyboardSensitivity": 20,
      "outputWebsocketUrl": "localhost:3000",
      "outputPolling": "100",
      "ledSensitivity": 20,
      "ledWebsocketUrl": "localhost:3001",
      "ledSerialPort": "COM5"
    }"#,
    )
    .unwrap()
  }

  fn load_saved() -> Option<Self> {
    let config_path = system::get_config_path()?;
    if !config_path.exists() {
      return None;
    }
    info!("Config file found at {:?}", config_path);
    let saved_data = fs::read_to_string(config_path.as_path()).ok()?;
    return Self::from_str(saved_data.as_str());
  }

  pub fn load() -> Self {
    Self::load_saved()
      .or_else(|| {
        warn!("Config loading from file failed, using default");
        Some(Self::default())
      })
      .unwrap()
  }

  pub fn save(&self) -> Option<()> {
    info!("Config saving...");
    let config_path = system::get_config_path()?;
    info!("Config saving to {:?}", config_path);
    fs::write(config_path.as_path(), self.raw.as_str()).unwrap();
    info!("Config saved");

    Some(())
  }
}
