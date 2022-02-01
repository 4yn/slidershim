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

pub struct LedJob {
  state: FullState,
  mode: LedMode,
}

impl LedJob {
  pub fn new(state: &FullState, mode: &LedMode) -> Self {
    Self {
      state: state.clone(),
      mode: mode.clone(),
    }
  }

  fn calc_lights(&self, flat_controller_state: Option<&Vec<bool>>, led_state: &mut LedState) {
    match self.mode {
      LedMode::Reactive { layout, .. } => {
        let flat_controller_state = flat_controller_state.unwrap();

        match layout {
          ReactiveLayout::Even { splits } => {
            let buttons_per_split = 32 / splits;

            let banks: Vec<bool> = flat_controller_state
              .chunks(32 / splits)
              .take(splits)
              .map(|x| x.contains(&true))
              .collect();

            for idx in 0..31 {
              led_state.paint(
                idx,
                match (idx + 1) % buttons_per_split {
                  0 => &[255, 0, 255],
                  _ => match banks[idx / buttons_per_split] {
                    true => &[255, 0, 255],
                    false => &[255, 255, 0],
                  },
                },
              );
            }
          }
          ReactiveLayout::Voltex => {
            led_state.led_state.fill(0);

            // Fixed
            led_state.paint(3, &[0, 0, 64]);
            for idx in 0..5 {
              led_state.paint(7 + idx * 4, &[64, 64, 64]);
            }
            led_state.paint(27, &[64, 0, 0]);

            // Left laser left
            if flat_controller_state[0..4].contains(&true) {
              for idx in 0..3 {
                led_state.paint(idx, &[0, 0, 255]);
              }
            };

            // Left laser right
            if flat_controller_state[4..8].contains(&true) {
              for idx in 4..7 {
                led_state.paint(idx, &[0, 0, 255]);
              }
            };

            // Right laser left
            if flat_controller_state[24..28].contains(&true) {
              for idx in 24..27 {
                led_state.paint(idx, &[255, 0, 0]);
              }
            };
            // Right laser right
            if flat_controller_state[28..32].contains(&true) {
              for idx in 28..31 {
                led_state.paint(idx, &[255, 0, 0]);
              }
            };

            // Buttons
            for (btn_idx, btn_banks) in flat_controller_state[8..24].chunks(4).enumerate() {
              if btn_banks.iter().skip(1).step_by(2).any(|x| *x) {
                led_state.paint(8 + btn_idx * 4, &[255, 255, 255]);
                led_state.paint(10 + btn_idx * 4, &[255, 255, 255]);
              }
            }

            // Fx
            for (fx_idx, fx_banks) in flat_controller_state[8..24].chunks(8).enumerate() {
              if fx_banks.iter().step_by(2).any(|x| *x) {
                led_state.paint(9 + fx_idx * 8, &[255, 0, 0]);
                led_state.paint(11 + fx_idx * 8, &[255, 0, 0]);
                led_state.paint(13 + fx_idx * 8, &[255, 0, 0]);
              }
            }
          }
        }
      }
      LedMode::Attract => {
        let now = Instant::now();
        let theta = (now - led_state.start).div_duration_f64(Duration::from_secs(4)) % 1.0;
        for idx in 0..31 {
          let slice_theta = (&theta + (idx as f64) / 32.0) % 1.0;
          let color = Srgb::from_color(Hsv::new(slice_theta * 360.0, 1.0, 1.0)).into_format::<u8>();
          led_state.paint(idx, &[color.red, color.green, color.blue]);
        }
      }
      _ => panic!("Not implemented"),
    }

    led_state.dirty = true;
  }
}

impl Job for LedJob {
  fn setup(&mut self) {}

  fn tick(&mut self) {
    let flat_controller_state: Option<Vec<bool>> = match self.mode {
      LedMode::Reactive { sensitivity, .. } => {
        let controller_state_handle = self.state.controller_state.lock().unwrap();
        Some(controller_state_handle.flat(&sensitivity))
      }
      _ => None,
    };

    {
      let mut led_state_handle = self.state.led_state.lock().unwrap();
      self.calc_lights(flat_controller_state.as_ref(), led_state_handle.deref_mut());
    }
    thread::sleep(Duration::from_millis(33));
  }

  fn teardown(&mut self) {}
}
