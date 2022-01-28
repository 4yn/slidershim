use crate::slider_io::{
  config::Config, controller_state::FullState, device::DeviceThread, led::LedThread,
};

pub struct Manager {
  state: FullState,
  config: Config,
  device_thread: DeviceThread,
  led_thread: LedThread,
}

impl Manager {
  pub fn new(config: Config) -> Self {
    let state = FullState::new();
    let device_thread = DeviceThread::new(&state, config.device_mode.clone());
    let led_thread = LedThread::new(&state, config.led_mode.clone());

    println!("Starting manager with config: {:?}", config);

    Self {
      state,
      config,
      device_thread,
      led_thread,
    }
  }
}
