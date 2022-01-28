use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
};

pub trait Job: Send {
  fn setup(&mut self);
  fn tick(&mut self);
  fn teardown(&mut self);
}

pub struct Worker {
  thread: Option<thread::JoinHandle<()>>,
  stop_signal: Arc<AtomicBool>,
}

impl Worker {
  pub fn new<T: 'static + Job>(mut job: T) -> Self {
    let stop_signal = Arc::new(AtomicBool::new(false));

    let stop_signal_clone = Arc::clone(&stop_signal);
    Self {
      thread: Some(thread::spawn(move || {
        job.setup();
        loop {
          job.tick();
          if stop_signal_clone.load(Ordering::SeqCst) {
            break;
          }
        }
        job.teardown();
      })),
      stop_signal,
    }
  }
}

impl Drop for Worker {
  fn drop(&mut self) {
    self.stop_signal.swap(true, Ordering::SeqCst);
    if self.thread.is_some() {
      self.thread.take().unwrap().join().ok();
    }
  }
}
