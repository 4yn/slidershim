extern crate slider_io;

use std::io;

use slider_io::{
  device::{brokenithm::BrokenithmJob, config::BrokenithmSpec},
  shared::worker::AsyncHaltableWorker,
  state::SliderState,
};

#[tokio::main]
async fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  let state = SliderState::new();

  let _worker = AsyncHaltableWorker::new(
    "brokenithm",
    BrokenithmJob::new(&state, &BrokenithmSpec::Nostalgia, &false, &1606),
  );
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
