extern crate slider_io;

use async_trait::async_trait;
use std::{future::Future, io, time::Duration};

use tokio::{select, time::sleep};

use slider_io::shared::worker::{AsyncHaltableJob, AsyncHaltableWorker};

struct CounterJob;

#[async_trait]
impl AsyncHaltableJob for CounterJob {
  async fn run<F: Future<Output = ()> + Send>(self, stop_signal: F) {
    let job_a = async {
      println!("Start job A");
      let mut x = 0;
      loop {
        x += 1;
        println!("{}", x);
        sleep(Duration::from_millis(500)).await;
      }
    };
    let job_b = async move {
      println!("Start job B");
      stop_signal.await;
      println!("Stop signal hit at job B");
    };

    select! {
      _ = job_a => {},
      _ = job_b => {},
    }
  }
}

#[tokio::main]
async fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  let _worker = AsyncHaltableWorker::new("counter", CounterJob);
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
