use crate::slider_io::{
  config::Config, controller_state::FullState, device::HidDeviceJob, led::LedJob,
  output::KeyboardOutputJob, worker::Worker,
};

pub struct Manager {
  state: FullState,
  config: Config,
  device_worker: Worker,
  output_worker: Worker,
  led_worker: Worker,
}

impl Manager {
  pub fn new(config: Config) -> Self {
    let state = FullState::new();
    let device_worker = Worker::new(HidDeviceJob::from_config(&state, &config.device_mode));
    let output_worker = Worker::new(KeyboardOutputJob::new(&state, &config.output_mode));
    let led_worker = Worker::new(LedJob::new(&state, &config.led_mode));

    println!("Starting manager with config: {:?}", config);

    Self {
      state,
      config,
      device_worker,
      output_worker,
      led_worker,
    }
  }
}
