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

use tokio::{runtime::Runtime, sync::oneshot, task};

pub trait ThreadJob: Send {
  fn setup(&mut self) -> bool;
  fn tick(&mut self);
  fn teardown(&mut self);
}

pub struct ThreadWorker {
  name: &'static str,
  thread: Option<thread::JoinHandle<()>>,
  stop_signal: Arc<AtomicBool>,
}

impl ThreadWorker {
  pub fn new<T: 'static + ThreadJob>(name: &'static str, mut job: T) -> Self {
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
          job.tick();
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
    if self.thread.is_some() {
      self.thread.take().unwrap().join().ok();
    }
  }
}

#[async_trait]
pub trait AsyncJob: Send + 'static {
  async fn run<F: Future<Output = ()> + Send>(self, stop_signal: F);
}

pub struct AsyncWorker {
  name: &'static str,
  runtime: Runtime,
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
    let runtime = tokio::runtime::Builder::new_multi_thread()
      .worker_threads(1)
      .enable_all()
      .build()
      .unwrap();

    let task = runtime.spawn(async move {
      job
        .run(async move {
          recv_stop.await.unwrap();
        })
        .await;
    });

    AsyncWorker {
      name,
      runtime,
      task: Some(task),
      stop_signal: Some(send_stop),
    }
  }
}

impl Drop for AsyncWorker {
  fn drop(&mut self) {
    info!("Async worker stopping {}", self.name);

    if self.stop_signal.is_some() {
      let send_stop = self.stop_signal.take().unwrap();
      send_stop.send(()).unwrap();
      // self.runtime.block_on(async move {
      //   send_stop.send(()).unwrap();
      // });
    }

    if self.task.is_some() {
      // let task = self.task.take().unwrap();
      // self.runtime.block_on(async move {
      //   task.await;
      //   info!("Async worker stopping internal {}", name);
      // });
    }
  }
}
