use std::{thread, time::Duration};

use crate::slider_io::{
  config::OutputMode, controller_state::FullState, keyboard::KeyboardOutput, worker::Job,
};

pub struct KeyboardOutputJob {
  state: FullState,
  sensitivity: u8,
  keyboard_output: KeyboardOutput,
}

impl KeyboardOutputJob {
  pub fn new(state: &FullState, mode: &OutputMode) -> Self {
    match mode {
      OutputMode::Keyboard {
        layout,
        sensitivity,
      } => Self {
        state: state.clone(),
        sensitivity: *sensitivity,
        keyboard_output: KeyboardOutput::new(layout.clone()),
      },
      _ => panic!("Not implemented"),
    }
  }
}

impl Job for KeyboardOutputJob {
  fn setup(&mut self) {}

  fn tick(&mut self) {
    let flat_controller_state: Vec<bool>;
    {
      let controller_state_handle = self.state.controller_state.lock().unwrap();
      flat_controller_state = controller_state_handle.flat(&self.sensitivity);
    }

    self.keyboard_output.tick(&flat_controller_state);
    thread::sleep(Duration::from_millis(10));
  }

  fn teardown(&mut self) {
    self.keyboard_output.reset();
  }
}
