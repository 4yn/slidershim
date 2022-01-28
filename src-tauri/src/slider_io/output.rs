use std::{
  ops::Deref,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread::{self, JoinHandle},
  time::Duration,
};

use crate::slider_io::{config::OutputMode, controller_state::FullState, keyboard};

pub struct OutputThread {
  thread: Option<JoinHandle<()>>,
  stop_signal: Arc<AtomicBool>,
}

impl OutputThread {
  pub fn new(state: &FullState, mode: OutputMode) -> Self {
    let controller_state = state.clone_controller();
    let stop_signal = Arc::new(AtomicBool::new(false));

    let stop_signal_clone = Arc::clone(&stop_signal);

    Self {
      thread: Some(match mode {
        OutputMode::None => thread::spawn(|| {}),
        OutputMode::Keyboard {
          layout,
          sensitivity,
        } => thread::spawn(move || {
          let mut keyboard_output = keyboard::KeyboardOutput::new(layout);
          loop {
            {
              let controller_state_handle = controller_state.lock().unwrap();
              keyboard_output.tick(controller_state_handle.deref(), &sensitivity);
            }

            {
              if stop_signal_clone.load(Ordering::SeqCst) {
                break;
              }
            }

            thread::sleep(Duration::from_millis(10));
          }

          keyboard_output.reset();
        }),
        OutputMode::Websocket { .. } => thread::spawn(|| {}),
      }),
      stop_signal,
    }
  }
}

impl Drop for OutputThread {
  fn drop(&mut self) {
    self.stop_signal.swap(true, Ordering::SeqCst);
    if self.thread.is_some() {
      self.thread.take().unwrap().join().ok();
    }
  }
}
