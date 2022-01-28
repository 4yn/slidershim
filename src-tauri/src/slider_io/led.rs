use std::{
  ops::DerefMut,
  thread,
  time::{Duration, Instant},
};

use palette::{FromColor, Hsv, Srgb};

use crate::slider_io::{
  config::{LedMode, ReactiveLayout},
  controller_state::{FullState, LedState},
  worker::Job,
};

type LedCallback = fn(&Vec<bool>, &mut LedState) -> ();

pub struct LedJob {
  state: FullState,
  mode: LedMode,
  sensitivity: u8,

  splits: usize,
  buttons_per_split: usize,
}

impl LedJob {
  pub fn new(state: &FullState, mode: &LedMode) -> Self {
    let splits = match mode {
      LedMode::Reactive { layout } => match layout {
        ReactiveLayout::Four => 4,
        ReactiveLayout::Eight => 8,
        ReactiveLayout::Sixteen => 16,
      },
      _ => 16,
    };

    Self {
      state: state.clone(),
      mode: mode.clone(),
      sensitivity: 20,

      splits,
      buttons_per_split: 32 / splits,
    }
  }

  fn calc_lights(&self, flat_controller_state: &Vec<bool>, led_state: &mut LedState) {
    match self.mode {
      LedMode::Reactive { .. } => {
        let banks: Vec<bool> = flat_controller_state
          .chunks(self.buttons_per_split)
          .take(self.splits)
          .map(|x| x.iter().any(|x| *x))
          .collect();

        led_state
          .led_state
          .chunks_mut(3)
          .enumerate()
          .for_each(|(idx, chunk)| match (idx + 1) % self.buttons_per_split {
            0 => {
              chunk[0] = 255;
              chunk[1] = 0;
              chunk[2] = 255;
            }
            _ => match banks[idx / self.buttons_per_split] {
              true => {
                chunk[0] = 255;
                chunk[1] = 0;
                chunk[2] = 255;
              }
              false => {
                chunk[0] = 255;
                chunk[1] = 255;
                chunk[2] = 0;
              }
            },
          })
      }
      LedMode::Attract => {
        let now = Instant::now();
        let theta = (now - led_state.start).div_duration_f64(Duration::from_secs(4)) % 1.0;
        led_state
          .led_state
          .chunks_mut(3)
          .enumerate()
          .for_each(|(idx, chunk)| {
            let slice_theta = (&theta + (idx as f64) / 31.0) % 1.0;
            let color =
              Srgb::from_color(Hsv::new(slice_theta * 360.0, 1.0, 1.0)).into_format::<u8>();
            chunk[0] = color.red;
            chunk[1] = color.green;
            chunk[2] = color.blue;
          });
      }
      _ => panic!("Not implemented"),
    }

    led_state.dirty = true;
  }
}

impl Job for LedJob {
  fn setup(&mut self) {}

  fn tick(&mut self) {
    let flat_controller_state: Vec<bool>;
    {
      let controller_state_handle = self.state.controller_state.lock().unwrap();
      flat_controller_state = controller_state_handle.flat(&self.sensitivity);
    }

    {
      let mut led_state_handle = self.state.led_state.lock().unwrap();
      self.calc_lights(&flat_controller_state, led_state_handle.deref_mut());
    }
    thread::sleep(Duration::from_millis(33));
  }

  fn teardown(&mut self) {}
}
