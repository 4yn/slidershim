use log::info;

use crate::slider_io::{
  brokenithm::BrokenithmJob,
  config::{Config, DeviceMode},
  controller_state::FullState,
  device::HidDeviceJob,
  led::LedJob,
  output::OutputJob,
  worker::{AsyncWorker, ThreadWorker},
};

#[allow(dead_code)]
pub struct Manager {
  state: FullState,
  config: Config,
  device_worker: Option<ThreadWorker>,
  brokenithm_worker: Option<AsyncWorker>,
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

    let (device_worker, brokenithm_worker) = match &config.device_mode {
      DeviceMode::Brokenithm { .. } => (
        None,
        Some(AsyncWorker::new("brokenithm", BrokenithmJob::new(&state))),
      ),
      other => (
        Some(ThreadWorker::new(
          "device",
          HidDeviceJob::from_config(&state, other),
        )),
        None,
      ),
    };
    let output_worker = ThreadWorker::new("output", OutputJob::new(&state, &config.output_mode));
    let led_worker = ThreadWorker::new("led", LedJob::new(&state, &config.led_mode));

    Self {
      state,
      config,
      device_worker,
      brokenithm_worker,
      output_worker,
      led_worker,
    }
  }
}
