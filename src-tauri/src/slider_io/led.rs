use std::{
  borrow::BorrowMut,
  ops::{Deref, DerefMut},
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread::{self, JoinHandle},
  time::{Duration, Instant},
};

use palette::{FromColor, Hsv, Srgb};

use crate::slider_io::{
  config::{LedMode, ReactiveLayout},
  controller_state::{ControllerState, FullState, LedState},
};

fn update_reactive(
  controller_state: &ControllerState,
  led_state: &mut LedState,
  reactive_layout: &ReactiveLayout,
  sensitivity: &u8,
) {
  let splits = match reactive_layout {
    ReactiveLayout::Four => 4,
    ReactiveLayout::Eight => 8,
    ReactiveLayout::Sixteen => 16,
  };
  let buttons_per_split = 32 / splits;

  let banks: Vec<bool> = controller_state
    .flat(sensitivity)
    .chunks(buttons_per_split)
    .take(splits)
    .map(|x| x.iter().any(|x| *x))
    .collect();

  // controller_state
  // .ground_state
  // .chunks(buttons_per_split)
  // .map(|x| x.iter().max().unwrap() > &sensitivity)
  // .collect();

  // (0..splits)
  //   .map(|i| {
  //     controller_state.ground_state[i * buttons_per_split..(i + 1) * buttons_per_split]
  //       .iter()
  //       .max()
  //       .unwrap()
  //       > &sensitivity
  //   })
  //   .collect();

  led_state
    .led_state
    .chunks_mut(3)
    .enumerate()
    .for_each(|(idx, chunk)| match (idx + 1) % buttons_per_split {
      0 => {
        chunk[0] = 255;
        chunk[1] = 0;
        chunk[2] = 255;
      }
      _ => match banks[idx / buttons_per_split] {
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
    });

  // println!("{:?}", controller_state.ground_state);
  // println!("{:?}", banks);
  // println!("{:?}", led_state.led_state);

  led_state.dirty = true;
}

fn update_attract(led_state: &mut LedState) {
  let now = Instant::now();
  let theta = (now - led_state.start).div_duration_f64(Duration::from_secs(4)) % 1.0;

  led_state
    .led_state
    .chunks_mut(3)
    .enumerate()
    .for_each(|(idx, chunk)| {
      let slice_theta = (&theta + (idx as f64) / 31.0) % 1.0;
      let color = Srgb::from_color(Hsv::new(slice_theta * 360.0, 1.0, 1.0)).into_format::<u8>();
      chunk[0] = color.red;
      chunk[1] = color.green;
      chunk[2] = color.blue;
    });

  // println!("{} {:?}", theta, led_state.led_state);

  led_state.dirty = true;
}

pub struct LedThread {
  thread: Option<JoinHandle<()>>,
  stop_signal: Arc<AtomicBool>,
}

impl LedThread {
  pub fn new(state: &FullState, mode: LedMode) -> Self {
    let stop_signal = Arc::new(AtomicBool::new(false));

    let stop_signal_clone = Arc::clone(&stop_signal);
    let controller_state = state.clone_controller();
    let led_state = state.clone_led();
    Self {
      thread: Some(thread::spawn(move || loop {
        // println!("Led thread: {:?}", mode);
        match &mode {
          LedMode::Reactive { layout } => {
            let controller_state_handle = controller_state.lock().unwrap();
            let mut led_state_handle = led_state.lock().unwrap();
            update_reactive(
              controller_state_handle.deref(),
              led_state_handle.deref_mut(),
              layout,
              &20,
            )
          }
          LedMode::Attract => {
            let mut led_state_handle = led_state.lock().unwrap();
            update_attract(led_state_handle.deref_mut());
          }
          _ => {}
        }

        {
          if stop_signal_clone.load(Ordering::SeqCst) {
            break;
          }
        }

        thread::sleep(Duration::from_millis(33))
      })),
      stop_signal,
    }
  }
}

impl Drop for LedThread {
  fn drop(&mut self) {
    self.stop_signal.swap(true, Ordering::SeqCst);
    if self.thread.is_some() {
      self.thread.take().unwrap().join().ok();
    }
  }
}
