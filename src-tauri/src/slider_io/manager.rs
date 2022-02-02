use log::info;

use crate::slider_io::{
  config::Config, controller_state::FullState, device::HidDeviceJob, led::LedJob,
  output::OutputJob, worker::Worker,
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
    info!("Starting manager");
    info!("Device config {:?}", config.device_mode);
    info!("Output config {:?}", config.output_mode);
    info!("LED config {:?}", config.led_mode);

    let state = FullState::new();
    let device_worker = Worker::new(HidDeviceJob::from_config(&state, &config.device_mode));
    let output_worker = Worker::new(OutputJob::new(&state, &config.output_mode));
    let led_worker = Worker::new(LedJob::new(&state, &config.led_mode));

    Self {
      state,
      config,
      device_worker,
      output_worker,
      led_worker,
    }
  }
}
