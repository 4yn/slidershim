use std::{thread, time::Duration};

use crate::slider_io::{
  config::OutputMode, controller_state::FullState, gamepad::GamepadOutput,
  keyboard::KeyboardOutput, worker::ThreadJob,
};

pub trait OutputHandler: Send {
  fn tick(&mut self, flat_controller_state: &Vec<bool>);
  fn reset(&mut self);
}

pub struct OutputJob {
  state: FullState,
  sensitivity: u8,
  handler: Box<dyn OutputHandler>,
}

impl OutputJob {
  pub fn new(state: &FullState, mode: &OutputMode) -> Self {
    match mode {
      OutputMode::Keyboard {
        layout,
        sensitivity,
      } => Self {
        state: state.clone(),
        sensitivity: *sensitivity,
        handler: Box::new(KeyboardOutput::new(layout.clone())),
      },
      OutputMode::Gamepad {
        layout,
        sensitivity,
      } => Self {
        state: state.clone(),
        sensitivity: *sensitivity,
        handler: Box::new(GamepadOutput::new(layout.clone())),
      },
      _ => panic!("Not implemented"),
    }
  }
}

impl ThreadJob for OutputJob {
  fn setup(&mut self) -> bool {
    true
  }

  fn tick(&mut self) {
    let flat_controller_state: Vec<bool>;
    {
      let controller_state_handle = self.state.controller_state.lock().unwrap();
      flat_controller_state = controller_state_handle.flat(&self.sensitivity);
    }

    self.handler.tick(&flat_controller_state);
    thread::sleep(Duration::from_millis(10));
  }

  fn teardown(&mut self) {
    self.handler.reset();
  }
}
