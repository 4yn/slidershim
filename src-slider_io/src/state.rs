use parking_lot::Mutex;
use std::{sync::Arc, time::Instant};

/// Stores the input state of a slider controller, including ground touch pads,
/// air strings and extra buttons.
pub struct SliderInput {
  /// Represents touch pressure in 32 touch pads in a 2 tall and 16 wide grid.
  /// Each pressur is in a `u8` from 0 to 255. Pads are represented in order of
  /// bottom left, then top left, then flows from left to right.
  pub ground: [u8; 32],

  /// Represents air string state in 6 pads starting from bottom to top. 0 means
  /// uninterrupted, 1 means interrupted.
  pub air: [u8; 6],

  /// Represents extra button state, usually used for coin/test/card entry
  /// functions.
  pub extra: [u8; 3],
}

impl SliderInput {
  /// Make a blank input state.
  pub fn new() -> Self {
    Self {
      ground: [0; 32],
      air: [0; 6],
      extra: [0; 3],
    }
  }

  /// Converts an input state to a `Vec<bool>`, used for output simulation and
  /// visualisation.
  pub fn to_flat(&self, sensitivity: &u8) -> Vec<bool> {
    self
      .ground
      .iter()
      .map(|x| x >= sensitivity)
      .chain(self.air.iter().chain(self.extra.iter()).map(|x| x > &0))
      .collect()
  }

  /// Flips the ground slider state vertically. Used when taking input for
  /// tasoller controllers as they report starting from top left (instead of
  /// botton left that is used internally).
  pub fn flip_vert(&mut self) {
    for i in 0..16 {
      self.ground.swap(i * 2, i * 2 + 1);
    }
  }
}

// Stores the lighting state of a slider controller.
pub struct SliderLights {
  /// Represents the RGB pixel values of the slider controller from left to
  /// right. Alternates between 16 touch pad pixels and 15 divider pixels.
  pub ground: [u8; 3 * 31],

  /// Internal dirty flag used to indicate that new lighting data is available.
  pub dirty: bool,

  /// To deprecate
  pub start: Instant,
}

impl SliderLights {
  /// Make a blank lighting state.
  pub fn new() -> Self {
    Self {
      ground: [0; 3 * 31],
      dirty: false,
      start: Instant::now(),
    }
  }

  /// Apply a RGB color to some pixel in the lighting state.
  pub fn paint(&mut self, idx: usize, color: &[u8; 3]) {
    self.ground[3 * idx..3 * (idx + 1)].copy_from_slice(color);
  }
}

/// Stores data required for a single slider controller. Data and lighting
/// states are stored seperately in their own `Arc<Mutex<T>>` so that they can
/// be locked independently.
pub struct SliderState {
  /// Input data for the slider controller.
  pub input: Arc<Mutex<SliderInput>>,

  /// Lighting data for the slider controller.
  pub lights: Arc<Mutex<SliderLights>>,
}

impl SliderState {
  /// Creates a blank slider controller state
  pub fn new() -> Self {
    Self {
      input: Arc::new(Mutex::new(SliderInput::new())),
      lights: Arc::new(Mutex::new(SliderLights::new())),
    }
  }

  /// Takes an instantaneous slider controller state (input + lighting) as a
  /// `Vec<u8>` that can be used for visualisation.
  pub fn snapshot(&self) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    {
      let input_handle = self.input.lock();
      buf.extend(input_handle.ground);
      buf.extend(input_handle.air);
      buf.extend(input_handle.extra);
    };
    {
      let lights_handle = self.lights.lock();
      buf.extend(lights_handle.ground);
    };

    buf
  }
}

impl Clone for SliderState {
  fn clone(&self) -> Self {
    Self {
      input: Arc::clone(&self.input),
      lights: Arc::clone(&self.lights),
    }
  }
}
