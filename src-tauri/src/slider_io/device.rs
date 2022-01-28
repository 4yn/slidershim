use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread::{self, JoinHandle},
};

use crate::slider_io::{config::DeviceMode, controller_state::FullState, hid};

pub struct DeviceThread {
  thread: Option<JoinHandle<()>>,
  stop_signal: Arc<AtomicBool>,
}

impl DeviceThread {
  pub fn new(state: &FullState, mode: DeviceMode) -> Self {
    let controller_state = state.clone_controller();
    let led_state = state.clone_led();
    let stop_signal = Arc::new(AtomicBool::new(false));

    let stop_signal_clone = Arc::clone(&stop_signal);
    Self {
      thread: Some(match mode {
        DeviceMode::None => thread::spawn(|| {}),
        DeviceMode::Tasoller { .. } => thread::spawn(|| {}),
        DeviceMode::Yuancon => thread::spawn(move || {
          hid::poll_controller(
            0x1973,
            0x2001,
            move |buf| {
              if (buf.len != 34) {
                return;
              }

              let mut controller_state_handle = controller_state.lock().unwrap();
              controller_state_handle
                .ground_state
                .clone_from_slice(&buf.data[2..34]);
              for i in 0..6 {
                controller_state_handle.air_state[i ^ 1] =
                  if buf.data[0] & (1 << i) == 0 { 1 } else { 0 };
              }
              for i in 0..3 {
                controller_state_handle.extra_state[i] =
                  if buf.data[1] & (1 << i) == 0 { 1 } else { 0 };
              }

              // println!("{:?}", controller_state_handle.ground_state);
            },
            move |buf| {
              let mut led_state_handle = led_state.lock().unwrap();
              if led_state_handle.dirty {
                buf.len = 31 * 2;
                buf
                  .data
                  .chunks_mut(2)
                  .take(31)
                  .zip(led_state_handle.led_state.chunks(3).rev())
                  .for_each(|(buf_chunk, state_chunk)| {
                    buf_chunk[0] = (state_chunk[0] << 3 & 0xe0) | (state_chunk[2] >> 3);
                    buf_chunk[1] = (state_chunk[1] & 0xf8) | (state_chunk[0] >> 5);
                  });
                led_state_handle.dirty = false;
              }
            },
            &stop_signal_clone,
          )
          // .unwrap_or_else(|err| {
          //   println!("Device thread: {:?}", err);
          // });
          .unwrap();
        }),
        DeviceMode::Brokenithm { .. } => thread::spawn(|| {}),
      }),
      stop_signal: stop_signal,
    }
  }
}

impl Drop for DeviceThread {
  fn drop(&mut self) {
    self.stop_signal.swap(true, Ordering::SeqCst);
    if self.thread.is_some() {
      self.thread.take().unwrap().join().ok();
    }
  }
}
