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
  Deemo,
  Voltex,
  Neardayo,
}

#[derive(Debug, Clone, Copy)]
pub enum GamepadLayout {
  Voltex,
  Neardayo,
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
