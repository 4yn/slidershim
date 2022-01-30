use std::{
  error,
  ops::{Deref, DerefMut},
  thread,
  time::Duration,
};

use log::{error, info};

use rusb::{self, DeviceHandle, GlobalContext};

use crate::slider_io::{
  config::DeviceMode,
  controller_state::{ControllerState, FullState, LedState},
  worker::Job,
};

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

type HidReadCallback = fn(&Buffer, &mut ControllerState) -> ();
type HidLedCallback = fn(&mut Buffer, &LedState) -> ();

enum WriteType {
  Bulk,
  Interrupt,
}

pub struct HidDeviceJob {
  state: FullState,
  vid: u16,
  pid: u16,
  read_endpoint: u8,
  led_endpoint: u8,

  read_callback: HidReadCallback,
  read_buf: Buffer,

  led_write_type: WriteType,
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
    led_type: WriteType,
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
      led_write_type: led_type,
      led_callback,
      led_buf: Buffer::new(),
      handle: None,
    }
  }

  pub fn from_config(state: &FullState, mode: &DeviceMode) -> Self {
    match mode {
      DeviceMode::TasollerOne => Self::new(
        state.clone(),
        0x1ccf,
        0x2333,
        0x84,
        0x03,
        |buf, controller_state| {
          if buf.len != 11 {
            return;
          }

          let bits: Vec<u8> = buf
            .data
            .iter()
            .flat_map(|x| (0..8).map(move |i| ((x) >> i) & 1))
            .collect();
          for i in 0..32 {
            controller_state.ground_state[i] = bits[34 + i] * 255;
          }

          controller_state.air_state.copy_from_slice(&bits[28..34]);
          controller_state.extra_state.copy_from_slice(&bits[26..28]);
        },
        WriteType::Bulk,
        |buf, led_state| {
          buf.len = 240;
          buf.data[0] = 'B' as u8;
          buf.data[1] = 'L' as u8;
          buf.data[2] = '\x00' as u8;
          for (buf_chunk, state_chunk) in buf.data[3..96]
            .chunks_mut(3)
            .take(31)
            .zip(led_state.led_state.chunks(3).rev())
          {
            buf_chunk[0] = state_chunk[2];
            buf_chunk[1] = state_chunk[1];
            buf_chunk[2] = state_chunk[0];
          }
          buf.data[96..240].fill(0);
        },
      ),
      DeviceMode::TasollerTwo => Self::new(
        state.clone(),
        0x1ccf,
        0x2333,
        0x84,
        0x03,
        |buf, controller_state| {
          if buf.len != 36 {
            return;
          }

          controller_state
            .ground_state
            .copy_from_slice(&buf.data[4..36]);

          let bits: Vec<u8> = (0..8).map(|x| (buf.data[3] >> x) & 1).collect();
          controller_state.air_state.copy_from_slice(&bits[2..8]);
          controller_state.extra_state.copy_from_slice(&bits[2..8]);
        },
        WriteType::Bulk,
        |buf, led_state| {
          buf.len = 240;
          buf.data[0] = 'B' as u8;
          buf.data[1] = 'L' as u8;
          buf.data[2] = '\x00' as u8;
          for (buf_chunk, state_chunk) in buf.data[3..96]
            .chunks_mut(3)
            .take(31)
            .zip(led_state.led_state.chunks(3).rev())
          {
            buf_chunk[0] = state_chunk[2];
            buf_chunk[1] = state_chunk[1];
            buf_chunk[2] = state_chunk[0];
          }
          buf.data[96..240].fill(0);
        },
      ),
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
            .copy_from_slice(&buf.data[2..34]);
          for i in 0..6 {
            controller_state.air_state[i ^ 1] = if buf.data[0] & (1 << i) == 0 { 1 } else { 0 };
          }
          for i in 0..3 {
            controller_state.extra_state[i] = if buf.data[1] & (1 << i) == 0 { 1 } else { 0 };
          }
        },
        WriteType::Interrupt,
        |buf, led_state| {
          buf.len = 31 * 2;
          for (buf_chunk, state_chunk) in buf
            .data
            .chunks_mut(2)
            .take(31)
            .zip(led_state.led_state.chunks(3).rev())
          {
            buf_chunk[0] = (state_chunk[0] << 3 & 0xe0) | (state_chunk[2] >> 3);
            buf_chunk[1] = (state_chunk[1] & 0xf8) | (state_chunk[0] >> 5);
          }
        },
      ),
      _ => panic!("Not implemented"),
    }
  }

  fn setup_impl(&mut self) -> Result<(), Box<dyn error::Error>> {
    info!("Device finding vid {} pid {}", self.vid, self.pid);
    let handle = rusb::open_device_with_vid_pid(self.vid, self.pid);
    if handle.is_none() {
      error!("Could not find device");
    }
    let mut handle = handle.unwrap();
    info!("Device found {:?}", handle);

    if handle.kernel_driver_active(0).unwrap_or(false) {
      info!("Device detaching kernel driver");
      handle.detach_kernel_driver(0)?;
    }
    info!("Device setting configuration");
    handle.set_active_configuration(1)?;
    info!("Device claiming interface");
    handle.claim_interface(0)?;
    self.handle = Some(handle);
    Ok(())
  }
}

const TIMEOUT: Duration = Duration::from_millis(20);

impl Job for HidDeviceJob {
  fn setup(&mut self) {
    self.setup_impl().unwrap();
  }

  fn tick(&mut self) {
    // Input loop
    let handle = self.handle.as_mut().unwrap();

    {
      let res = handle
        .read_interrupt(self.read_endpoint, &mut self.read_buf.data, TIMEOUT)
        .unwrap_or(0);
      self.read_buf.len = res;
      if self.read_buf.len != 0 {
        let mut controller_state_handle = self.state.controller_state.lock().unwrap();
        (self.read_callback)(&self.read_buf, controller_state_handle.deref_mut());
      }
    }

    // Led loop
    {
      {
        let mut led_state_handle = self.state.led_state.lock().unwrap();
        if led_state_handle.dirty {
          (self.led_callback)(&mut self.led_buf, led_state_handle.deref());
          led_state_handle.dirty = false;
        }
      }

      if self.led_buf.len != 0 {
        let res = (match self.led_write_type {
          WriteType::Bulk => handle.write_bulk(self.led_endpoint, &self.led_buf.data, TIMEOUT),
          WriteType::Interrupt => {
            handle.write_interrupt(self.led_endpoint, &self.led_buf.data, TIMEOUT)
          }
        })
        .unwrap_or(0);
        if res == self.led_buf.len + 1 {
          self.led_buf.len = 0;
        }
      }
    }

    // thread::sleep(Duration::from_millis(10));
  }

  fn teardown(&mut self) {
    let handle = self.handle.as_mut().unwrap();
    handle.release_interface(0).ok();
  }
}
