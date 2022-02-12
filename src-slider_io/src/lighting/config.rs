use serde_json::Value;

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

impl LedMode {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(match v["ledMode"].as_str()? {
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
      _ => return None,
    })
  }
}
