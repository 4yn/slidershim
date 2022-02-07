use log::error;
use std::time::Duration;

use crate::slider_io::{
  config::OutputMode, controller_state::FullState, gamepad::GamepadOutput,
  keyboard::KeyboardOutput, worker::ThreadJob,
};

pub trait OutputHandler: Send {
  fn tick(&mut self, flat_controller_state: &Vec<bool>) -> bool;
  fn reset(&mut self);
}

pub struct OutputJob {
  state: FullState,
  mode: OutputMode,
  t: u64,
  sensitivity: u8,
  handler: Option<Box<dyn OutputHandler>>,
}

impl OutputJob {
  pub fn new(state: &FullState, mode: &OutputMode) -> Self {
    Self {
      state: state.clone(),
      mode: mode.clone(),
      t: 0,
      sensitivity: 0,
      handler: None,
    }
  }
}

impl ThreadJob for OutputJob {
  fn setup(&mut self) -> bool {
    match self.mode {
      OutputMode::Keyboard {
        layout,
        polling,
        sensitivity,
      } => {
        self.t = polling.to_t_u64();
        self.sensitivity = sensitivity;
        self.handler = Some(Box::new(KeyboardOutput::new(layout.clone())));

        true
      }
      OutputMode::Gamepad {
        layout,
        polling,
        sensitivity,
      } => {
        self.t = polling.to_t_u64();
        self.sensitivity = sensitivity;
        let handler = GamepadOutput::new(layout.clone());
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

  fn tick(&mut self) -> bool {
    let flat_controller_state: Vec<bool>;
    {
      let controller_state_handle = self.state.controller_state.lock().unwrap();
      flat_controller_state = controller_state_handle.flat(&self.sensitivity);
    }

    if let Some(handler) = self.handler.as_mut() {
      handler.tick(&flat_controller_state);
    }
    spin_sleep::sleep(Duration::from_micros(self.t));

    true
  }

  fn teardown(&mut self) {
    if let Some(handler) = self.handler.as_mut() {
      handler.reset();
    }
  }
}
