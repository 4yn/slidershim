use log::{info, warn};
use serde_json::Value;
use std::fs;

use crate::{
  device::config::DeviceMode, lighting::config::LightsMode, output::config::OutputMode, system,
};

#[derive(Debug, Clone)]
pub struct Config {
  pub raw: String,
  pub device_mode: DeviceMode,
  pub output_mode: OutputMode,
  pub lights_mode: LightsMode,
}

impl Config {
  pub fn from_str(s: &str) -> Option<Config> {
    let v: Value = serde_json::from_str(s).ok()?;

    Some(Config {
      raw: s.to_string(),
      device_mode: DeviceMode::from_serde_value(&v)?,
      output_mode: OutputMode::from_serde_value(&v)?,
      lights_mode: LightsMode::from_serde_value(&v)?,
    })
  }

  fn default() -> Self {
    Self::from_str(
      r##"{
      "deviceMode": "none",
      "outputMode": "none",
      "ledMode": "none",
      "disableAirStrings": false,
      "divaSerialPort": "COM1",
      "divaBrightness": 63,
      "brokenithmPort": 1606,
      "keyboardSensitivity": 20,
      "keyboardDirectInput": false,
      "outputPolling": "100",
      "outputWebsocketUrl": "localhost:3000",
      "ledFaster": false,
      "ledColorActive": "#ff00ff",
      "ledColorInactive": "#ffff00",
      "ledColorAirActive": "#0086ed",
      "ledColorAirInactive": "#000000",
      "ledSensitivity": 20,
      "ledWebsocketUrl": "localhost:3001",
      "ledSerialPort": "COM5"
    }"##,
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
    let t = Self::load_saved();
    warn!("{:?}", t);
    t.or_else(|| {
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
