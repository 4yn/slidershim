use directories::ProjectDirs;
use image::Luma;
use log::{info, warn};
use qrcode::QrCode;
use serde_json::Value;
use std::{error::Error, fs, path::PathBuf};

use crate::{device::config::DeviceMode, lighting::config::LedMode, output::config::OutputMode};

pub fn list_ips() -> Result<Vec<String>, Box<dyn Error>> {
  let mut ips = vec![];
  for adapter in ipconfig::get_adapters()? {
    for ip_address in adapter.ip_addresses() {
      ips.push(format!("{}", ip_address));
    }
  }

  Ok(ips)
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
      device_mode: DeviceMode::from_serde_value(&v)?,
      output_mode: OutputMode::from_serde_value(&v)?,
      led_mode: LedMode::from_serde_value(&v)?,
    })
  }

  pub fn get_log_file_path() -> Option<Box<PathBuf>> {
    let project_dir = ProjectDirs::from("me", "impress labs", "slidershim").unwrap();
    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir).ok()?;

    let log_path = config_dir.join("log.txt");

    return Some(Box::new(log_path));
  }

  pub fn get_brokenithm_qr_path() -> Option<Box<PathBuf>> {
    let project_dir = ProjectDirs::from("me", "impress labs", "slidershim").unwrap();
    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir).ok()?;

    let brokenithm_qr_path = config_dir.join("brokenithm.png");

    let ips = list_ips().ok()?;
    let link = "http://imp.ress.me/t/sshelper?d=".to_string()
      + &ips
        .into_iter()
        .filter(|s| s.as_str().chars().filter(|x| *x == '.').count() == 3)
        .map(|s| base64::encode_config(s, base64::URL_SAFE_NO_PAD))
        .collect::<Vec<String>>()
        .join(";");
    let qr = QrCode::new(link).ok()?;
    let image = qr.render::<Luma<u8>>().build();
    image.save(brokenithm_qr_path.as_path()).ok()?;

    return Some(Box::new(brokenithm_qr_path));
  }

  fn get_config_path() -> Option<Box<PathBuf>> {
    let project_dir = ProjectDirs::from("me", "impress labs", "slidershim").unwrap();
    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir).ok()?;

    let config_path = config_dir.join("config.json");

    return Some(Box::new(config_path));
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
    let config_path = Self::get_config_path()?;
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
    let config_path = Self::get_config_path()?;
    info!("Config saving to {:?}", config_path);
    fs::write(config_path.as_path(), self.raw.as_str()).unwrap();
    info!("Config saved");

    Some(())
  }
}
