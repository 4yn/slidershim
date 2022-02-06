use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};

use crate::slider_io::{config::GamepadLayout, output::OutputHandler, voltex::VoltexState};

pub struct GamepadOutput {
  target: Xbox360Wired<Client>,
  use_air: bool,
  gamepad: XGamepad,
}

impl GamepadOutput {
  pub fn new(layout: GamepadLayout) -> Self {
    let client = Client::connect().unwrap();
    let use_air = match layout {
      GamepadLayout::Neardayo => true,
      _ => false,
    };
    let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
    target.plugin().unwrap();
    target.wait_ready().unwrap();
    Self {
      target,
      use_air,
      gamepad: XGamepad::default(),
    }
  }
}

impl OutputHandler for GamepadOutput {
  fn tick(&mut self, flat_controller_state: &Vec<bool>) {
    let voltex_state = VoltexState::from_flat(flat_controller_state);

    let buttons = voltex_state
      .bt
      .iter()
      .chain(voltex_state.fx.iter())
      .chain(voltex_state.extra.iter())
      .zip([
        XButtons::A,
        XButtons::B,
        XButtons::X,
        XButtons::Y,
        XButtons::LB,
        XButtons::RB,
        XButtons::START,
        XButtons::BACK,
        XButtons::GUIDE,
      ])
      .fold(0, |buttons, (state, code)| {
        buttons
          | match state {
            true => code,
            false => 0,
          }
      });

    let lx = (match voltex_state.laser[0] || (self.use_air && flat_controller_state[34]) {
      true => -30000,
      false => 0,
    } + match voltex_state.laser[1] || (self.use_air && flat_controller_state[35]) {
      true => 30000,
      false => 0,
    });

    let rx = (match voltex_state.laser[2] || (self.use_air && flat_controller_state[36]) {
      true => -30000,
      false => 0,
    } + match voltex_state.laser[3] || (self.use_air && flat_controller_state[37]) {
      true => 30000,
      false => 0,
    });

    let mut dirty = false;
    if self.gamepad.buttons.raw != buttons {
      self.gamepad.buttons.raw = buttons;
      dirty = true;
    }
    if self.gamepad.thumb_lx != lx {
      self.gamepad.thumb_lx = lx;
      dirty = true;
    }
    if self.gamepad.thumb_rx != rx {
      self.gamepad.thumb_rx = rx;
      dirty = true;
    }

    if dirty {
      self.target.update(&self.gamepad).unwrap();
    }
  }

  fn reset(&mut self) {
    self.gamepad = XGamepad::default();
    self.target.update(&self.gamepad).unwrap();
  }
}

impl Drop for GamepadOutput {
  fn drop(&mut self) {
    self.target.unplug().unwrap();
  }
}

// dammit vigem_client::Event
unsafe impl Send for GamepadOutput {}
