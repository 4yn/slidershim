use std::{error::Error, fmt};

pub struct Buffer {
  pub data: [u8; 256],
  pub len: usize,
}

impl Buffer {
  pub fn new() -> Self {
    Buffer {
      data: [0; 256],
      len: 0,
    }
  }

  fn slice(&self) -> &[u8] {
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
