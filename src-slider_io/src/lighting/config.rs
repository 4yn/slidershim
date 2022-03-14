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
    layout: ReactiveLayout,
    sensitivity: u8,
  },
  Attract,
  Websocket {
    url: String,
  },
  Serial {
    port: String,
  },
}

impl LightsMode {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(match v["ledMode"].as_str()? {
      "none" => LightsMode::None,
      "reactive-4" => LightsMode::Reactive {
        layout: ReactiveLayout::Even { splits: 4 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-8" => LightsMode::Reactive {
        layout: ReactiveLayout::Even { splits: 8 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-16" => LightsMode::Reactive {
        layout: ReactiveLayout::Even { splits: 16 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-rainbow" => LightsMode::Reactive {
        layout: ReactiveLayout::Rainbow,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "reactive-voltex" => LightsMode::Reactive {
        layout: ReactiveLayout::Voltex,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
      },
      "attract" => LightsMode::Attract,
      "websocket" => LightsMode::Websocket {
        url: v["ledWebsocketUrl"].as_str()?.to_string(),
      },
      "serial" => LightsMode::Serial {
        port: v["ledSerialPort"].as_str()?.to_string(),
      },
      _ => return None,
    })
  }
}
