use atomic_float::AtomicF64;
use log::info;
use std::sync::{atomic::Ordering, Arc};

use crate::{
  config::Config,
  device::{brokenithm::BrokenithmJob, config::DeviceMode, diva::DivaSliderJob, hid::HidJob},
  lighting::{config::LightsMode, lighting::LightsJob, umgr_websocket::UmgrWebsocketJob},
  output::{config::OutputMode, output::OutputJob},
  shared::{
    utils::LoopTimer,
    worker::{AsyncHaltableWorker, AsyncWorker, ThreadWorker},
  },
  state::SliderState,
};

#[allow(dead_code)]
pub struct Context {
  state: SliderState,
  config: Config,
  device_thread_worker: Option<ThreadWorker>,
  device_async_worker: Option<AsyncWorker>,
  device_async_haltable_worker: Option<AsyncHaltableWorker>,
  output_worker: Option<AsyncWorker>,
  lights_worker: Option<AsyncWorker>,
  lights_haltable_worker: Option<AsyncHaltableWorker>,
  timers: Vec<(&'static str, Arc<AtomicF64>)>,
}

impl Context {
  pub fn new(config: Config) -> Self {
    info!("Context creating");
    info!("Device config {:?}", config.device_mode);
    info!("Output config {:?}", config.output_mode);
    info!("Lights config {:?}", config.lights_mode);

    let state = SliderState::new();
    let mut timers = vec![];

    let (device_thread_worker, device_async_worker, device_async_haltable_worker) =
      match &config.device_mode {
        DeviceMode::None => (None, None, None),
        DeviceMode::Brokenithm {
          spec,
          lights_enabled,
          port,
        } => (
          None,
          None,
          Some(AsyncHaltableWorker::new(
            "brokenithm",
            BrokenithmJob::new(&state, spec, lights_enabled, port),
          )),
        ),
        DeviceMode::Hardware { spec, disable_air } => (
          {
            let timer = LoopTimer::new();
            timers.push(("d", timer.fork()));
            Some(ThreadWorker::new(
              "device",
              HidJob::from_config(&state, spec, disable_air),
              timer,
            ))
          },
          None,
          None,
        ),
        DeviceMode::DivaSlider { port, brightness } => (
          {
            let timer = LoopTimer::new();
            timers.push(("d", timer.fork()));
            Some(ThreadWorker::new(
              "diva",
              DivaSliderJob::new(&state, port, *brightness),
              timer,
            ))
          },
          None,
          None,
        ),
      };
    let output_worker = match &config.output_mode {
      OutputMode::None => None,
      _ => {
        let timer = LoopTimer::new();
        timers.push(("o", timer.fork()));
        Some(AsyncWorker::new(
          "output",
          OutputJob::new(&state, &config.output_mode),
          timer,
        ))
      }
    };
    let (lights_worker, lights_haltable_worker) = match &config.lights_mode {
      LightsMode::None => (None, None),
      LightsMode::UmgrWebsocket { faster, port } => (
        None,
        Some(AsyncHaltableWorker::new(
          "lights",
          UmgrWebsocketJob::new(&state, faster, port),
        )),
      ),
      _ => {
        let timer = LoopTimer::new();
        timers.push(("l", timer.fork()));
        (
          Some(AsyncWorker::new(
            "lights",
            LightsJob::new(&state, &config.lights_mode),
            timer,
          )),
          None,
        )
      }
    };

    Self {
      state,
      config,
      device_thread_worker,
      device_async_worker,
      device_async_haltable_worker,
      output_worker,
      lights_worker,
      lights_haltable_worker,
      timers,
    }
  }

  pub fn clone_state(&self) -> SliderState {
    self.state.clone()
  }

  pub fn timer_state(&self) -> String {
    self
      .timers
      .iter()
      .map(|(s, f)| format!("{}:{:.1}/s", s, f.load(Ordering::SeqCst)))
      .collect::<Vec<String>>()
      .join(" ")
  }
}
