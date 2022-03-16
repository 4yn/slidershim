use serde_json::Value;

#[derive(Debug, Clone)]
pub enum HardwareSpec {
  TasollerOne,
  TasollerTwo,
  Yuancon,
}

#[derive(Debug, Clone)]
pub enum DeviceMode {
  None,
  Hardware {
    spec: HardwareSpec,
    disable_air: bool,
  },
  Brokenithm {
    ground_only: bool,
    lights_enabled: bool,
  },
  DivaSlider {
    port: String,
    brightness: u8,
  },
}

impl DeviceMode {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(match v["deviceMode"].as_str()? {
      "none" => DeviceMode::None,
      "tasoller-one" => DeviceMode::Hardware {
        spec: HardwareSpec::TasollerOne,
        disable_air: v["disableAirStrings"].as_bool()?,
      },
      "tasoller-two" => DeviceMode::Hardware {
        spec: HardwareSpec::TasollerTwo,
        disable_air: v["disableAirStrings"].as_bool()?,
      },
      "yuancon" => DeviceMode::Hardware {
        spec: HardwareSpec::Yuancon,
        disable_air: v["disableAirStrings"].as_bool()?,
      },
      "diva" => DeviceMode::DivaSlider {
        port: v["divaSerialPort"].as_str()?.to_string(),
        brightness: u8::try_from(v["divaBrightness"].as_i64()?).ok()?,
      },
      "brokenithm" => DeviceMode::Brokenithm {
        ground_only: v["disableAirStrings"].as_bool()?,
        lights_enabled: false,
      },
      "brokenithm-led" => DeviceMode::Brokenithm {
        ground_only: v["disableAirStrings"].as_bool()?,
        lights_enabled: true,
      },
      _ => return None,
    })
  }
}
