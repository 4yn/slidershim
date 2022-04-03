pub struct HoriState {
  pub slider: [bool; 16],
  pub bt: [bool; 4],
  pub extra: [bool; 1],
}

impl HoriState {
  pub fn from_flat(flat_input: &Vec<bool>) -> Self {
    let mut hori_state = Self {
      slider: [false; 16],
      bt: [false; 4],
      extra: [false; 1],
    };

    for (idx, i) in flat_input[0..32].iter().enumerate() {
      match idx % 2 {
        0 => {
          hori_state.bt[idx / 8] |= *i;
        }
        1 => {
          hori_state.slider[idx / 2] |= *i;
        }
        _ => unreachable!(),
      }
    }

    hori_state.extra[0] = flat_input[38];

    hori_state
  }

  pub fn from_flat_to_wide(flat_input: &Vec<bool>) -> Self {
    let mut hori_state = Self {
      slider: [false; 16],
      bt: [false; 4],
      extra: [false; 1],
    };

    for (idx, i) in flat_input[0..32].iter().enumerate() {
      hori_state.slider[idx / 2] |= *i;
    }

    hori_state.extra[0] = flat_input[38];

    hori_state
  }
}
