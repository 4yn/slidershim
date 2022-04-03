use log::error;
use std::error::Error;
use vigem_client::{Client, DS4Report, DualShock4Wired, TargetId};

use crate::shared::hori::HoriState;

use super::{config::HoriLayout, output::OutputHandler};

pub struct HoriOutput {
  target: DualShock4Wired<Client>,
  slider_only: bool,
  gamepad: DS4Report,
}

impl HoriOutput {
  pub fn new(layout: HoriLayout) -> Option<Self> {
    let target = Self::get_target();

    let slider_only = match layout {
      HoriLayout::Full => false,
      HoriLayout::SliderOnly => true,
    };

    match target {
      Ok(target) => Some(Self {
        target,
        slider_only,
        gamepad: DS4Report::default(),
      }),
      Err(e) => {
        error!("Gamepad connection error: {}", e);
        error!("Gamepad connection error: Is ViGEMBus missing?");
        None
      }
    }
  }

  fn get_target() -> Result<DualShock4Wired<Client>, Box<dyn Error>> {
    let client = Client::connect()?;

    let mut target = DualShock4Wired::new(client, TargetId::DUALSHOCK4_WIRED);
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

impl OutputHandler for HoriOutput {
  fn tick(&mut self, flat_input: &Vec<bool>) -> bool {
    let hori_state = match self.slider_only {
      false => HoriState::from_flat(flat_input),
      true => HoriState::from_flat_to_wide(flat_input)
    };

    let buttons: u16 = hori_state
      .bt
      .iter()
      .zip([
        // https://github.com/ViGEm/ViGEmClient/blob/master/include/ViGEm/Common.h#L117
        1 << 7, // triangle
        1 << 4, // square
        1 << 5, // cross
        1 << 6, // circle
      ])
      .fold(0x8, |buttons, (state, code)| {
        buttons
          | match state {
            true => code,
            false => 0,
          }
      });

    let axis: u32 = hori_state
      .slider
      .iter()
      .enumerate()
      .fold(0, |axis, (idx, state)| {
        axis
          | match state {
            true => 0b11 << ((15 - idx) * 2),
            false => 0,
          }
      })
      ^ 0x80808080;

    let mut dirty = false;
    if self.gamepad.buttons != buttons {
      self.gamepad.buttons = buttons;
      dirty = true;
    }

    for (idx, state) in [
      &mut self.gamepad.thumb_lx,
      &mut self.gamepad.thumb_ly,
      &mut self.gamepad.thumb_rx,
      &mut self.gamepad.thumb_ry,
    ]
    .into_iter()
    .enumerate()
    {
      let slice: u8 = ((axis >> ((3 - idx) * 8)) & 0xff) as u8;
      if *state != slice {
        *state = slice;
        dirty = true;
      }
    }

    match dirty {
      true => self.update(),
      false => true,
    }
  }

  fn reset(&mut self) {
    self.gamepad = DS4Report::default();
    self.update();
  }
}

impl Drop for HoriOutput {
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
unsafe impl Send for HoriOutput {}
