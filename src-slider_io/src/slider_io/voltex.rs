pub struct VoltexState {
  pub laser: [bool; 4],
  pub bt: [bool; 4],
  pub fx: [bool; 2],
  pub extra: [bool; 3],
}

impl VoltexState {
  pub fn from_flat(flat_controller_state: &Vec<bool>) -> Self {
    let mut voltex_state = Self {
      laser: [false; 4],
      bt: [false; 4],
      fx: [false; 2],
      extra: [false; 3],
    };

    voltex_state.laser[0] = flat_controller_state[0..4].contains(&true);
    voltex_state.laser[1] = flat_controller_state[4..8].contains(&true);
    voltex_state.laser[2] = flat_controller_state[24..28].contains(&true);
    voltex_state.laser[3] = flat_controller_state[28..32].contains(&true);

    for i in 0..4 {
      voltex_state.bt[i] = flat_controller_state[9 + i * 4] || flat_controller_state[11 + i * 4];
    }

    for i in 0..2 {
      voltex_state.fx[i] = flat_controller_state[8 + i * 8]
        || flat_controller_state[10 + i * 8]
        || flat_controller_state[12 + i * 8]
        || flat_controller_state[14 + i * 8];
    }

    for i in 0..3 {
      voltex_state.extra[i] = flat_controller_state[38 + i];
    }

    voltex_state
  }
}
