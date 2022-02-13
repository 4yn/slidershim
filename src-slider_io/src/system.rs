use directories::ProjectDirs;
use image::Luma;
use qrcode::QrCode;
use std::{error::Error, fs, path::PathBuf};

pub fn list_ips() -> Result<Vec<String>, Box<dyn Error>> {
  let mut ips = vec![];
  for adapter in ipconfig::get_adapters()? {
    for ip_address in adapter.ip_addresses() {
      ips.push(format!("{}", ip_address));
    }
  }

  Ok(ips)
}

/// Get the %APPDATA% path for config files (and create if it does not already
/// exist).
fn get_config_dir() -> Option<Box<PathBuf>> {
  let project_dir = ProjectDirs::from("me", "impress labs", "slidershim").unwrap();
  let config_dir = project_dir.config_dir();
  fs::create_dir_all(config_dir).ok()?;

  Some(Box::new(config_dir.to_path_buf()))
}

/// Generates a helper QR for connecting with brokenithm
pub fn get_brokenithm_qr_path() -> Option<Box<PathBuf>> {
  let config_dir = get_config_dir()?;
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

pub fn get_log_file_path() -> Option<Box<PathBuf>> {
  let config_dir = get_config_dir()?;
  let log_path = config_dir.join("log.txt");

  return Some(Box::new(log_path));
}

pub fn get_config_path() -> Option<Box<PathBuf>> {
  let config_dir = get_config_dir()?;
  let config_path = config_dir.join("config.json");

  return Some(Box::new(config_path));
}
