use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub enum ReactiveLayout {
  Even { splits: usize },
  Voltex,
  Rainbow,
}

#[derive(Debug, Clone)]
pub enum LightsMode {
  None,
  Reactive {
    faster: bool,
    layout: ReactiveLayout,
    sensitivity: u8,
  },
  Attract {
    faster: bool,
  },
  Websocket {
    faster: bool,
    url: String,
  },
  Serial {
    faster: bool,
    port: String,
  },
}

impl LightsMode {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(match v["ledMode"].as_str()? {
      "none" => LightsMode::None,
      "reactive-4" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Even { splits: 4 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-8" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Even { splits: 8 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-16" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Even { splits: 16 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-rainbow" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Rainbow,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-voltex" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Voltex,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "attract" => LightsMode::Attract {
        faster: v["ledFaster"].as_bool()?,
      },
      "websocket" => LightsMode::Websocket {
        faster: v["ledFaster"].as_bool()?,
        url: v["ledWebsocketUrl"].as_str()?.to_string(),
      },
      "serial" => LightsMode::Serial {
        faster: v["ledFaster"].as_bool()?,
        port: v["ledSerialPort"].as_str()?.to_string(),
      },
      _ => return None,
    })
  }
}
