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
  },
  Brokenithm {
    ground_only: bool,
    led_enabled: bool,
  },
}

impl DeviceMode {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(match v["deviceMode"].as_str()? {
      "none" => DeviceMode::None,
      "tasoller-one" => DeviceMode::Hardware {
        spec: HardwareSpec::TasollerOne,
      },
      "tasoller-two" => DeviceMode::Hardware {
        spec: HardwareSpec::TasollerTwo,
      },
      "yuancon" => DeviceMode::Hardware {
        spec: HardwareSpec::Yuancon,
      },
      "brokenithm" => DeviceMode::Brokenithm {
        ground_only: false,
        led_enabled: false,
      },
      "brokenithm-led" => DeviceMode::Brokenithm {
        ground_only: false,
        led_enabled: true,
      },
      "brokenithm-ground" => DeviceMode::Brokenithm {
        ground_only: true,
        led_enabled: false,
      },
      "brokenithm-ground-led" => DeviceMode::Brokenithm {
        ground_only: true,
        led_enabled: true,
      },
      _ => return None,
    })
  }
}
