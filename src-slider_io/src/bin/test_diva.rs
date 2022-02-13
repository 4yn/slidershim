extern crate slider_io;

use std::io;

use slider_io::{
  device::diva,
  shared::{utils::LoopTimer, worker::ThreadWorker},
  state::SliderState,
};

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  let state = SliderState::new();

  let timer = LoopTimer::new();
  let _worker = ThreadWorker::new(
    "d",
    diva::DivaSliderJob::new(&state, &"COM5".to_string()),
    timer,
  );

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
