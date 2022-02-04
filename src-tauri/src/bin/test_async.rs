extern crate slidershim;

use std::{io, time::Duration};

use tokio::time::sleep;

// use slidershim::slider_io::worker::{AsyncJob, AsyncJobFut, AsyncJobRecvStop, AsyncWorker};

// struct CounterJob;

// impl AsyncJob for CounterJob {
//   fn job(self, mut recv_stop: AsyncJobRecvStop) -> AsyncJobFut {
//     return Box::pin(async move {
//       let mut x = 0;
//       loop {
//         x += 1;
//         println!("{}", x);
//         sleep(Duration::from_millis(500)).await;
//         match recv_stop.try_recv() {
//           Ok(_) => {
//             println!("@@@");
//             break;
//           }
//           _ => {}
//         }
//       }
//     });
//   }
// }

fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  // let worker = AsyncWorker::new("counter", CounterJob);
  let mut input = String::new();
  let string = io::stdin().read_line(&mut input).unwrap();
}
