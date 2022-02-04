extern crate slidershim;

use std::{io, time::Duration};

use tokio::time::sleep;

// use slidershim::slider_io::{
//   brokenithm::BrokenithmJob, controller_state::FullState, worker::AsyncWorker,
// };

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  // let worker = AsyncWorker::new("brokenithm", BrokenithmJob::new(FullState::new()));
  let mut input = String::new();
  let string = io::stdin().read_line(&mut input).unwrap();
}
