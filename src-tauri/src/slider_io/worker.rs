use async_trait::async_trait;
use log::info;
use std::{
  future::Future,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
};

use tokio::{sync::oneshot, task};

use crate::slider_io::utils::LoopTimer;

pub trait ThreadJob: Send {
  fn setup(&mut self) -> bool;
  fn tick(&mut self) -> bool;
  fn teardown(&mut self);
}

pub struct ThreadWorker {
  name: &'static str,
  thread: Option<thread::JoinHandle<()>>,
  stop_signal: Arc<AtomicBool>,
}

impl ThreadWorker {
  pub fn new<T: 'static + ThreadJob>(name: &'static str, mut job: T, mut timer: LoopTimer) -> Self {
    info!("Thread worker starting {}", name);

    let stop_signal = Arc::new(AtomicBool::new(false));

    let stop_signal_clone = Arc::clone(&stop_signal);
    Self {
      name,
      thread: Some(thread::spawn(move || {
        let setup_res = job.setup();
        stop_signal_clone.store(!setup_res, Ordering::SeqCst);

        loop {
          if stop_signal_clone.load(Ordering::SeqCst) {
            break;
          }
          if job.tick() {
            timer.tick();
          }
        }
        info!("Thread worker stopping internal {}", name);
        job.teardown();
      })),
      stop_signal,
    }
  }
}

impl Drop for ThreadWorker {
  fn drop(&mut self) {
    info!("Thread worker stopping {}", self.name);

    self.stop_signal.store(true, Ordering::SeqCst);
    if let Some(thread) = self.thread.take() {
      thread.join().ok();
    };
  }
}

#[async_trait]
pub trait AsyncJob: Send + 'static {
  async fn run<F: Future<Output = ()> + Send>(self, stop_signal: F);
}

pub struct AsyncWorker {
  name: &'static str,
  task: Option<task::JoinHandle<()>>,
  stop_signal: Option<oneshot::Sender<()>>,
}

impl AsyncWorker {
  pub fn new<T>(name: &'static str, job: T) -> AsyncWorker
  where
    T: AsyncJob,
  {
    info!("Async worker starting {}", name);

    let (send_stop, recv_stop) = oneshot::channel::<()>();

    let task = tokio::spawn(async move {
      job
        .run(async move {
          recv_stop.await.ok();
        })
        .await;
    });

    AsyncWorker {
      name,
      task: Some(task),
      stop_signal: Some(send_stop),
    }
  }
}

impl Drop for AsyncWorker {
  fn drop(&mut self) {
    info!("Async worker stopping {}", self.name);

    if let Some(stop_signal) = self.stop_signal.take() {
      stop_signal.send(()).ok();
    }
    self.task.take();
  }
}
