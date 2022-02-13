extern crate slider_io;

use std::{io, thread::sleep, time::Duration};

use slider_io::shared::{
  utils::LoopTimer,
  worker::{ThreadJob, ThreadWorker},
};

struct TestJob {
  data: i64,
}

impl ThreadJob for TestJob {
  fn setup(&mut self) -> bool {
    self.data = 0;
    println!("setup {}", self.data);
    true
  }
  fn tick(&mut self) -> bool {
    self.data += 1;
    println!("tick {}", self.data);
    sleep(Duration::from_millis(500));
    true
  }
}

impl Drop for TestJob {
  fn drop(&mut self) {
    self.data = -1;
    println!("teardown {}", self.data);
  }
}

fn main() {
  let timer = LoopTimer::new();
  let _worker = ThreadWorker::new("j", TestJob { data: 1 }, timer);

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
