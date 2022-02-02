use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
};

pub trait Job: Send {
  fn setup(&mut self) -> bool;
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
        let setup_res = job.setup();
        stop_signal_clone.store(!setup_res, Ordering::SeqCst);

        loop {
          if stop_signal_clone.load(Ordering::SeqCst) {
            break;
          }
          job.tick();
        }
        job.teardown();
      })),
      stop_signal,
    }
  }
}

impl Drop for Worker {
  fn drop(&mut self) {
    self.stop_signal.store(true, Ordering::SeqCst);
    if self.thread.is_some() {
      self.thread.take().unwrap().join().ok();
    }
  }
}
