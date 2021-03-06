use log::error;
use std::error::Error;
use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};

use crate::shared::voltex::VoltexState;

use super::{config::GamepadLayout, output::OutputHandler};

struct LastWind {
  left: bool,
  right: bool,
  out: i16,
}

impl LastWind {
  fn new() -> Self {
    LastWind {
      left: false,
      right: false,
      out: 0,
    }
  }

  fn update(&mut self, left: bool, right: bool) -> i16 {
    let out = match (left, right) {
      (false, false) => 0,
      (true, false) => -1,
      (false, true) => 1,
      (true, true) => match (self.left, self.right) {
        (false, false) => 0,
        (true, false) => 1,
        (false, true) => -1,
        (true, true) => self.out,
      },
    };

    self.left = left;
    self.right = right;
    self.out = out;

    out
  }
}

pub struct GamepadOutput {
  target: Xbox360Wired<Client>,
  use_air: bool,
  gamepad: XGamepad,
  left_wind: LastWind,
  right_wind: LastWind,
}

impl GamepadOutput {
  pub fn new(layout: GamepadLayout) -> Option<Self> {
    let target = Self::get_target();
    let use_air = match layout {
      GamepadLayout::Neardayo => true,
      _ => false,
    };

    match target {
      Ok(target) => Some(Self {
        target,
        use_air,
        gamepad: XGamepad::default(),
        left_wind: LastWind::new(),
        right_wind: LastWind::new(),
      }),
      Err(e) => {
        error!("Gamepad connection error: {}", e);
        error!("Gamepad connection error: Is ViGEMBus missing?");
        None
      }
    }
  }

  fn get_target() -> Result<Xbox360Wired<Client>, Box<dyn Error>> {
    let client = Client::connect()?;

    let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
    target.plugin()?;
    target.wait_ready()?;
    Ok(target)
  }

  fn update(&mut self) -> bool {
    match self.target.update(&self.gamepad) {
      Ok(_) => true,
      Err(e) => {
        error!("Gamepad update error: {}", e);
        false
      }
    }
  }
}

impl OutputHandler for GamepadOutput {
  fn tick(&mut self, flat_input: &Vec<bool>) -> bool {
    let voltex_state = VoltexState::from_flat(flat_input);

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

    let lx = self.left_wind.update(
      voltex_state.laser[0] || (self.use_air && flat_input[32]),
      voltex_state.laser[1] || (self.use_air && (flat_input[33] || flat_input[34])),
    ) * 20000;

    let rx = self.right_wind.update(
      voltex_state.laser[2] || (self.use_air && (flat_input[35] || flat_input[36])),
      voltex_state.laser[3] || (self.use_air && flat_input[37]),
    ) * 20000;

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

    match dirty {
      true => self.update(),
      false => true,
    }
  }

  fn reset(&mut self) {
    self.gamepad = XGamepad::default();
    self.update();
  }
}

impl Drop for GamepadOutput {
  fn drop(&mut self) {
    match self.target.unplug() {
      Ok(_) => {}
      Err(e) => {
        error!("Gamepad unplug error: {}", e);
      }
    }
  }
}

// dammit vigem_client::Event
unsafe impl Send for GamepadOutput {}
