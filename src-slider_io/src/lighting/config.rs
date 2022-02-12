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
