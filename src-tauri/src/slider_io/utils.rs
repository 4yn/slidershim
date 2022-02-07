use atomic_float::AtomicF64;
use std::{
  error::Error,
  fmt,
  sync::{atomic::Ordering, Arc},
  time::{Duration, Instant},
};

pub struct Buffer {
  pub data: [u8; 256],
  pub len: usize,
}

#[allow(dead_code)]
impl Buffer {
  pub fn new() -> Self {
    Buffer {
      data: [0; 256],
      len: 0,
    }
  }

  pub fn slice(&self) -> &[u8] {
    &self.data[0..self.len]
  }
}

#[derive(Debug)]
pub struct ShimError;

impl<'a> fmt::Display for ShimError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ShimError")
  }
}

impl Error for ShimError {
  fn description(&self) -> &str {
    "shimError"
  }
}

pub fn list_ips() -> Result<Vec<String>, Box<dyn Error>> {
  let mut ips = vec![];
  for adapter in ipconfig::get_adapters()? {
    for ip_address in adapter.ip_addresses() {
      ips.push(format!("{}", ip_address));
    }
  }

  Ok(ips)
}

pub struct LoopTimer {
  cap: usize,
  cur: usize,
  buf: Vec<Instant>,
  freq: Arc<AtomicF64>,
}

impl LoopTimer {
  pub fn new() -> Self {
    Self {
      cap: 100,
      cur: 0,
      buf: vec![Instant::now() - Duration::from_secs(10); 100],
      freq: Arc::new(AtomicF64::new(0.0)),
    }
  }
  pub fn tick(&mut self) {
    let last = self.buf[self.cur];
    let now = Instant::now();
    self.buf[self.cur] = now;

    let delta = (now - last) / 100 + Duration::from_micros(1);
    let freq = Duration::from_millis(1000)
      .div_duration_f64(delta)
      .clamp(0.0, 9999.0);
    self.freq.store(freq, Ordering::SeqCst);

    self.cur = match self.cur + 1 {
      cur if cur == self.cap => 0,
      cur => cur,
    }
  }

  // pub fn reset(&mut self) {
  //   self.buf = vec![Instant::now(); 100];
  // }

  pub fn fork(&self) -> Arc<AtomicF64> {
    Arc::clone(&self.freq)
  }
}
