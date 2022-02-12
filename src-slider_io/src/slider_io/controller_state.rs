use parking_lot::Mutex;
use std::{sync::Arc, time::Instant};

pub struct ControllerState {
  pub ground_state: [u8; 32],
  pub air_state: [u8; 6],
  pub extra_state: [u8; 3],
}

impl ControllerState {
  pub fn new() -> Self {
    Self {
      ground_state: [0; 32],
      air_state: [0; 6],
      extra_state: [0; 3],
    }
  }

  pub fn flat(&self, sensitivity: &u8) -> Vec<bool> {
    self
      .ground_state
      .iter()
      .map(|x| x > sensitivity)
      .chain(
        self
          .air_state
          .iter()
          .chain(self.extra_state.iter())
          .map(|x| x > &0),
      )
      .collect()
  }

  pub fn flip_vert(&mut self) {
    for i in 0..16 {
      self.ground_state.swap(i * 2, i * 2 + 1);
    }
  }
}

pub struct LedState {
  pub led_state: [u8; 3 * 31],
  pub dirty: bool,
  pub start: Instant,
}

impl LedState {
  pub fn new() -> Self {
    Self {
      led_state: [0; 3 * 31],
      dirty: false,
      start: Instant::now(),
    }
  }

  pub fn paint(&mut self, idx: usize, color: &[u8; 3]) {
    self.led_state[3 * idx..3 * (idx + 1)].copy_from_slice(color);
  }
}

pub struct FullState {
  pub controller_state: Arc<Mutex<ControllerState>>,
  pub led_state: Arc<Mutex<LedState>>,
}

impl FullState {
  pub fn new() -> Self {
    Self {
      controller_state: Arc::new(Mutex::new(ControllerState::new())),
      led_state: Arc::new(Mutex::new(LedState::new())),
    }
  }

  pub fn clone_controller(&self) -> Arc<Mutex<ControllerState>> {
    Arc::clone(&self.controller_state)
  }

  pub fn clone_led(&self) -> Arc<Mutex<LedState>> {
    Arc::clone(&self.led_state)
  }

  pub fn snapshot(&self) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    {
      let controller_state_handle = self.controller_state.lock();
      buf.extend(controller_state_handle.ground_state);
      buf.extend(controller_state_handle.air_state);
      buf.extend(controller_state_handle.extra_state);
    };
    {
      let led_state_handle = self.led_state.lock();
      buf.extend(led_state_handle.led_state);
    };

    buf
  }
}

impl Clone for FullState {
  fn clone(&self) -> Self {
    Self {
      controller_state: self.clone_controller(),
      led_state: self.clone_led(),
    }
  }
}
