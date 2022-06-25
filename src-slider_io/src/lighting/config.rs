use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub enum ReactiveLayout {
  Even { splits: usize },
  Six,
  Voltex,
  Hori,
  Rainbow,
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
  pub active: [u8; 3],
  pub inactive: [u8; 3],
}

impl ColorScheme {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(Self {
      active: [
        u8::from_str_radix(&v["ledColorActive"].as_str()?[1..3], 16).ok()?,
        u8::from_str_radix(&v["ledColorActive"].as_str()?[3..5], 16).ok()?,
        u8::from_str_radix(&v["ledColorActive"].as_str()?[5..7], 16).ok()?,
      ],
      inactive: [
        u8::from_str_radix(&v["ledColorInactive"].as_str()?[1..3], 16).ok()?,
        u8::from_str_radix(&v["ledColorInactive"].as_str()?[3..5], 16).ok()?,
        u8::from_str_radix(&v["ledColorInactive"].as_str()?[5..7], 16).ok()?,
      ],
    })
  }

  pub fn default() -> Self {
    Self {
      active: [255, 0, 255],
      inactive: [255, 255, 0],
    }
  }

  pub fn from_serde_value_or_default(v: &Value) -> Self {
    Self::from_serde_value(v)
      .or(Some(Self {
        active: [255, 0, 255],
        inactive: [255, 255, 0],
      }))
      .unwrap()
  }
}

#[derive(Debug, Clone)]
pub enum LightsMode {
  None,
  Reactive {
    faster: bool,
    layout: ReactiveLayout,
    sensitivity: u8,
    color: ColorScheme,
  },
  Attract {
    faster: bool,
  },
  Websocket {
    faster: bool,
    url: String,
  },
  UmgrWebsocket {
    faster: bool,
    port: u16,
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
      "reactive-16" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Even { splits: 16 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::from_serde_value_or_default(v),
      },
      "reactive-8" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Even { splits: 8 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::from_serde_value_or_default(v),
      },
      "reactive-6" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Six,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::from_serde_value_or_default(v),
      },
      "reactive-4" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Even { splits: 4 },
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::from_serde_value_or_default(v),
      },
      "reactive-rainbow" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Rainbow,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::default(),
      },
      "reactive-voltex" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Voltex,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::default(),
      },
      "reactive-hori" => LightsMode::Reactive {
        faster: v["ledFaster"].as_bool()?,
        layout: ReactiveLayout::Hori,
        sensitivity: u8::try_from(v["ledSensitivity"].as_i64()?).ok()?,
        color: ColorScheme::default(),
      },
      "attract" => LightsMode::Attract {
        faster: v["ledFaster"].as_bool()?,
      },
      "websocket" => LightsMode::Websocket {
        faster: v["ledFaster"].as_bool()?,
        url: v["ledWebsocketUrl"].as_str()?.to_string(),
      },
      "umgr-websocket" => LightsMode::UmgrWebsocket {
        faster: v["ledFaster"].as_bool()?,
        port: u16::try_from(v["ledUmgrWebsocketPort"].as_i64()?).ok()?,
      },
      "serial" => LightsMode::Serial {
        faster: v["ledFaster"].as_bool()?,
        port: v["ledSerialPort"].as_str()?.to_string(),
      },
      _ => return None,
    })
  }
}
