use crate::slider_io::{
  config::Config, controller_state::FullState, device::DeviceThread, device_job::HidDeviceJob,
  led::LedThread, worker::Worker,
};

pub struct Manager {
  state: FullState,
  config: Config,
  // device_thread: DeviceThread,
  device_worker: Worker,
  led_thread: LedThread,
}

impl Manager {
  pub fn new(config: Config) -> Self {
    let state = FullState::new();
    // let device_thread = DeviceThread::new(&state, config.device_mode.clone());
    let device_worker = Worker::new(HidDeviceJob::from_config(&config.device_mode, &state));
    let led_thread = LedThread::new(&state, config.led_mode.clone());

    println!("Starting manager with config: {:?}", config);

    Self {
      state,
      config,
      // device_thread,
      device_worker,
      led_thread,
    }
  }
}
