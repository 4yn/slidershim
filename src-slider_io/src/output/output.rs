use async_trait::async_trait;
use log::error;
use std::time::Duration;
use tokio::time::{interval, Interval};

// use crate::slider_io::{
//   config::OutputMode, controller_state::FullState, gamepad::GamepadOutput,
//   keyboard::KeyboardOutput, worker::AsyncJob,
// };

use crate::{controller_state::FullState, shared::worker::AsyncJob};

use super::{config::OutputMode, gamepad::GamepadOutput, keyboard::KeyboardOutput};

pub trait OutputHandler: Send {
  fn tick(&mut self, flat_controller_state: &Vec<bool>) -> bool;
  fn reset(&mut self);
}

pub struct OutputJob {
  state: FullState,
  mode: OutputMode,
  sensitivity: u8,
  handler: Option<Box<dyn OutputHandler>>,
  timer: Interval,
}

impl OutputJob {
  pub fn new(state: &FullState, mode: &OutputMode) -> Self {
    Self {
      state: state.clone(),
      mode: mode.clone(),
      sensitivity: 0,
      handler: None,
      timer: interval(Duration::MAX),
    }
  }
}

#[async_trait]
impl AsyncJob for OutputJob {
  async fn setup(&mut self) -> bool {
    match self.mode {
      OutputMode::Keyboard {
        layout,
        polling,
        sensitivity,
      } => {
        self.sensitivity = sensitivity;
        self.handler = Some(Box::new(KeyboardOutput::new(layout.clone())));
        self.timer = interval(Duration::from_micros(polling.to_t_u64()));

        true
      }
      OutputMode::Gamepad {
        layout,
        polling,
        sensitivity,
      } => {
        self.sensitivity = sensitivity;
        let handler = GamepadOutput::new(layout.clone());
        self.timer = interval(Duration::from_micros(polling.to_t_u64()));

        match handler {
          Some(handler) => {
            self.handler = Some(Box::new(handler));
            true
          }
          None => false,
        }
      }
      _ => {
        error!("Not implemented");
        false
      }
    }
  }

  async fn tick(&mut self) -> bool {
    let flat_controller_state: Vec<bool>;
    {
      let controller_state_handle = self.state.controller_state.lock();
      flat_controller_state = controller_state_handle.to_flat(&self.sensitivity);
    }

    if let Some(handler) = self.handler.as_mut() {
      handler.tick(&flat_controller_state);
    }
    self.timer.tick().await;

    true
  }
}

impl Drop for OutputJob {
  fn drop(&mut self) {
    if let Some(handler) = self.handler.as_mut() {
      handler.reset();
    }
  }
}
