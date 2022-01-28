use std::{error, ops::DerefMut, time::Duration};

use rusb::{self, DeviceHandle, GlobalContext};

use crate::slider_io::{
  config::DeviceMode,
  controller_state::{ControllerState, FullState, LedState},
  worker::Job,
};

pub struct Buffer {
  pub data: [u8; 128],
  pub len: usize,
}

impl Buffer {
  pub fn new() -> Self {
    Buffer {
      data: [0; 128],
      len: 0,
    }
  }

  fn slice(&self) -> &[u8] {
    &self.data[0..self.len]
  }
}

type HidReadCallback = fn(&Buffer, &mut ControllerState) -> ();
type HidLedCallback = fn(&mut Buffer, &mut LedState) -> ();

pub struct HidDeviceJob {
  state: FullState,
  vid: u16,
  pid: u16,
  read_endpoint: u8,
  led_endpoint: u8,

  read_callback: HidReadCallback,
  read_buf: Buffer,

  led_callback: HidLedCallback,
  led_buf: Buffer,

  handle: Option<DeviceHandle<GlobalContext>>,
}

impl HidDeviceJob {
  fn new(
    state: FullState,
    vid: u16,
    pid: u16,
    read_endpoint: u8,
    led_endpoint: u8,
    read_callback: HidReadCallback,
    led_callback: HidLedCallback,
  ) -> Self {
    Self {
      state,
      vid,
      pid,
      read_endpoint,
      led_endpoint,
      read_callback,
      read_buf: Buffer::new(),
      led_callback,
      led_buf: Buffer::new(),
      handle: None,
    }
  }

  pub fn from_config(mode: &DeviceMode, state: &FullState) -> Self {
    match mode {
      DeviceMode::Yuancon => Self::new(
        state.clone(),
        0x1973,
        0x2001,
        0x81,
        0x02,
        |buf, controller_state| {
          if buf.len != 34 {
            return;
          }

          controller_state
            .ground_state
            .clone_from_slice(&buf.data[2..34]);
          for i in 0..6 {
            controller_state.air_state[i ^ 1] = if buf.data[0] & (1 << i) == 0 { 1 } else { 0 };
          }
          for i in 0..3 {
            controller_state.extra_state[i] = if buf.data[1] & (1 << i) == 0 { 1 } else { 0 };
          }
        },
        |buf, led_state| {
          if !led_state.dirty {
            return;
          }
          buf.len = 31 * 2;
          buf
            .data
            .chunks_mut(2)
            .take(31)
            .zip(led_state.led_state.chunks(3).rev())
            .for_each(|(buf_chunk, state_chunk)| {
              buf_chunk[0] = (state_chunk[0] << 3 & 0xe0) | (state_chunk[2] >> 3);
              buf_chunk[1] = (state_chunk[1] & 0xf8) | (state_chunk[0] >> 5);
            });
          led_state.dirty = false;
        },
      ),
      _ => panic!("Not implemented"),
    }
  }

  fn setup_impl(&mut self) -> Result<(), Box<dyn error::Error>> {
    let mut handle = rusb::open_device_with_vid_pid(self.vid, self.pid).unwrap();
    if handle.kernel_driver_active(0).unwrap_or(false) {
      handle.detach_kernel_driver(0)?;
    }
    handle.set_active_configuration(1)?;
    handle.claim_interface(0)?;
    self.handle = Some(handle);
    Ok(())
  }
}

const timeout: Duration = Duration::from_millis(20);

impl Job for HidDeviceJob {
  fn setup(&mut self) {
    self.setup_impl().unwrap();
  }

  fn tick(&mut self) {
    // Input loop
    let handle = self.handle.as_mut().unwrap();

    {
      let res = handle
        .read_interrupt(self.read_endpoint, &mut self.read_buf.data, timeout)
        .unwrap_or(0);
      self.read_buf.len = res;
      if self.read_buf.len != 0 {
        let mut controller_state_handle = self.state.controller_state.lock().unwrap();
        (self.read_callback)(&self.read_buf, controller_state_handle.deref_mut());
      }
    }

    // Led loop
    {
      let mut led_state_handle = self.state.led_state.lock().unwrap();
      (self.led_callback)(&mut self.led_buf, led_state_handle.deref_mut());
      if self.led_buf.len != 0 {
        let res = handle
          .write_interrupt(self.led_endpoint, &self.led_buf.data, timeout)
          .unwrap_or(0);
        if res == self.led_buf.len + 1 {
          self.led_buf.len = 0;
        }
      }
    }
  }

  fn teardown(&mut self) {
    let handle = self.handle.as_mut().unwrap();
    handle.release_interface(0).ok();
  }
}
