extern crate slider_io;

use std::{
  io,
  time::{Duration, Instant},
};

use slider_io::{device::diva, shared::worker::ThreadJob, state::SliderState};

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  let state = SliderState::new();
  let mut job = diva::DivaSliderJob::new(&state, &"COM1".to_string(), 0x3f);

  let ok = job.setup();
  while ok {
    job.tick();
  }

  // let state = SliderState::new();

  // let timer = LoopTimer::new();
  // let _worker = ThreadWorker::new(
  //   "d",
  //   diva::DivaSliderJob::new(&state, &"COM4".to_string(), 0x3f),
  //   timer,
  // );

  println!("Press enter to quit");
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}
