use log::info;

use crate::slider_io::{
  config::Config, controller_state::FullState, device::HidDeviceJob, led::LedJob,
  output::OutputJob, worker::ThreadWorker,
};

pub struct Manager {
  state: FullState,
  config: Config,
  device_worker: ThreadWorker,
  output_worker: ThreadWorker,
  led_worker: ThreadWorker,
}

impl Manager {
  pub fn new(config: Config) -> Self {
    info!("Starting manager");
    info!("Device config {:?}", config.device_mode);
    info!("Output config {:?}", config.output_mode);
    info!("LED config {:?}", config.led_mode);

    let state = FullState::new();
    let device_worker = ThreadWorker::new(
      "device",
      HidDeviceJob::from_config(&state, &config.device_mode),
    );
    let output_worker = ThreadWorker::new("output", OutputJob::new(&state, &config.output_mode));
    let led_worker = ThreadWorker::new("led", LedJob::new(&state, &config.led_mode));

    Self {
      state,
      config,
      device_worker,
      output_worker,
      led_worker,
    }
  }
}
