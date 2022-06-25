use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub enum PollingRate {
  Sixty,
  Hundred,
  TwoHundredFifty,
  FiveHundred,
  Thousand,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyboardLayout {
  Tasoller,
  Yuancon,
  Umiguri,
  TasollerHalf,
  EightK,
  SixK,
  FourK,
  Voltex,
  Neardayo,
}

#[derive(Debug, Clone, Copy)]
pub enum GamepadLayout {
  Voltex,
  Neardayo,
}

#[derive(Debug, Clone, Copy)]
pub enum HoriLayout {
  Full,
  SliderOnly,
}

#[derive(Debug, Clone)]
pub enum OutputMode {
  None,
  Keyboard {
    layout: KeyboardLayout,
    polling: PollingRate,
    sensitivity: u8,
  },
  Gamepad {
    layout: GamepadLayout,
    polling: PollingRate,
    sensitivity: u8,
  },
  Hori {
    layout: HoriLayout,
    polling: PollingRate,
    sensitivity: u8,
  },
  Websocket {
    url: String,
    polling: PollingRate,
  },
}

impl PollingRate {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "60" => Some(PollingRate::Sixty),
      "100" => Some(PollingRate::Hundred),
      "250" => Some(PollingRate::TwoHundredFifty),
      "500" => Some(PollingRate::FiveHundred),
      "1000" => Some(PollingRate::Thousand),
      _ => None,
    }
  }

  pub fn to_t_u64(&self) -> u64 {
    match self {
      PollingRate::Sixty => 16666,
      PollingRate::Hundred => 10000,
      PollingRate::TwoHundredFifty => 4000,
      PollingRate::FiveHundred => 2000,
      PollingRate::Thousand => 1000,
    }
  }
}

impl OutputMode {
  pub fn from_serde_value(v: &Value) -> Option<Self> {
    Some(match v["outputMode"].as_str().unwrap() {
      "none" => OutputMode::None,
      "kb-32-tasoller" => OutputMode::Keyboard {
        layout: KeyboardLayout::Tasoller,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-32-yuancon" => OutputMode::Keyboard {
        layout: KeyboardLayout::Yuancon,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-32-umiguri" => OutputMode::Keyboard {
        layout: KeyboardLayout::Umiguri,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-16" => OutputMode::Keyboard {
        layout: KeyboardLayout::TasollerHalf,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-8" => OutputMode::Keyboard {
        layout: KeyboardLayout::EightK,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-6" => OutputMode::Keyboard {
        layout: KeyboardLayout::SixK,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-4" => OutputMode::Keyboard {
        layout: KeyboardLayout::FourK,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-voltex" => OutputMode::Keyboard {
        layout: KeyboardLayout::Voltex,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "kb-neardayo" => OutputMode::Keyboard {
        layout: KeyboardLayout::Neardayo,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "gamepad-voltex" => OutputMode::Gamepad {
        layout: GamepadLayout::Voltex,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "gamepad-neardayo" => OutputMode::Gamepad {
        layout: GamepadLayout::Neardayo,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "gamepad-hori" => OutputMode::Hori {
        layout: HoriLayout::Full,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "gamepad-hori-wide" => OutputMode::Hori {
        layout: HoriLayout::SliderOnly,
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
        sensitivity: u8::try_from(v["keyboardSensitivity"].as_i64()?).ok()?,
      },
      "websocket" => OutputMode::Websocket {
        url: v["outputWebsocketUrl"].as_str()?.to_string(),
        polling: PollingRate::from_str(v["outputPolling"].as_str()?)?,
      },
      _ => return None,
    })
  }
}
