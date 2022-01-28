extern crate slidershim;

use std::io;

// use slidershim::slider_io::worker::{Job, Worker};

// struct TestJob {
//   data: i64,
// }

// impl Job for TestJob {
//   fn setup(&mut self) {
//     self.data = 10;
//     println!("setup {}", self.data);
//   }
//   fn tick(&mut self) {
//     self.data -= 1;
//     println!("tick {}", self.data);
//   }
//   fn teardown(&mut self) {
//     self.data = 11;
//     println!("teardown {}", self.data);
//   }
// }

fn main() {
  // let worker = Worker::new(TestJob { data: 1 });

  let mut input = String::new();
  let string = io::stdin().read_line(&mut input).unwrap();
}
