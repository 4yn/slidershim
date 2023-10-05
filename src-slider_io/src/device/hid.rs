use log::{error, info};
use rusb::{self, DeviceHandle, GlobalContext};
use std::{
  borrow::BorrowMut,
  error::Error,
  mem::swap,
  ops::{Deref, DerefMut},
  time::Duration,
};

use lzfx::compress;

use crate::{
  shared::{
    utils::{Buffer, ShimError},
    worker::ThreadJob,
  },
  state::{SliderInput, SliderLights, SliderState},
};

use super::config::HardwareSpec;

type HidReadCallback = fn(&Buffer, &mut SliderInput) -> ();
type HidLedCallback = fn(&mut Vec<LedSpec>, &SliderLights) -> ();

enum WriteType {
  Bulk,
  Interrupt,
}

pub struct LedSpec {
  led_write_type: WriteType,
  led_endpoint: u8,
  led_buf: Buffer,
}

impl LedSpec {
  fn new(led_write_type: WriteType, led_endpoint: u8) -> Self {
    Self {
      led_write_type,
      led_endpoint,
      led_buf: Buffer::new(),
    }
  }
}

pub struct HidJob {
  state: SliderState,

  vid: u16,
  pid: u16,
  disable_air: bool,

  read_endpoint: u8,
  read_callback: HidReadCallback,
  read_buf: Buffer,
  last_read_buf: Buffer,

  led_specs: Vec<LedSpec>,
  led_callback: HidLedCallback,

  handle: Option<DeviceHandle<GlobalContext>>,
}

impl HidJob {
  fn new(
    state: SliderState,
    vid: u16,
    pid: u16,
    disable_air: bool,

    read_endpoint: u8,
    read_callback: HidReadCallback,

    led_specs: Vec<LedSpec>,
    led_callback: HidLedCallback,
  ) -> Self {
    Self {
      state,
      vid,
      pid,
      disable_air,

      read_callback,
      read_endpoint,
      read_buf: Buffer::new(),
      last_read_buf: Buffer::new(),

      led_callback,
      led_specs,

      handle: None,
    }
  }

  pub fn from_config(state: &SliderState, spec: &HardwareSpec, disable_air: &bool) -> Self {
    match spec {
      HardwareSpec::TasollerOne => Self::new(
        state.clone(),
        0x1ccf,
        0x2333,
        *disable_air,
        0x84,
        |buf, input| {
          if buf.len != 11 {
            return;
          }

          let bits: Vec<u8> = buf
            .data
            .iter()
            .flat_map(|x| (0..8).map(move |i| ((x) >> i) & 1))
            .collect();
          for i in 0..32 {
            input.ground[i] = bits[34 + i] * 255;
          }
          input.flip_vert();

          input.air.copy_from_slice(&bits[28..34]);
          input.extra[0..2].copy_from_slice(&bits[26..28]);
        },
        vec![LedSpec::new(WriteType::Bulk, 0x03)],
        |led_specs, lights| {
          let buf = led_specs[0].led_buf.borrow_mut();
          buf.len = 240;
          buf.data[0] = 'B' as u8;
          buf.data[1] = 'L' as u8;
          buf.data[2] = '\x00' as u8;
          for (buf_chunk, state_chunk) in buf.data[3..96]
            .chunks_mut(3)
            .take(31)
            .zip(lights.ground.chunks(3).rev())
          {
            buf_chunk[0] = state_chunk[1];
            buf_chunk[1] = state_chunk[0];
            buf_chunk[2] = state_chunk[2];
          }
          buf.data[96..240].fill(0);
        },
      ),
      HardwareSpec::TasollerTwo => Self::new(
        state.clone(),
        0x1ccf,
        0x2333,
        *disable_air,
        0x84,
        |buf, input| {
          if buf.len != 36 {
            return;
          }

          input.ground.copy_from_slice(&buf.data[4..36]);
          input.flip_vert();

          let bits: Vec<u8> = (0..8).map(|x| (buf.data[3] >> x) & 1).collect();
          input.air.copy_from_slice(&bits[0..6]);
          input.extra[0..2].copy_from_slice(&bits[6..8]);
        },
        vec![LedSpec::new(WriteType::Bulk, 0x03)],
        |led_specs, lights| {
          let buf = led_specs[0].led_buf.borrow_mut();
          buf.len = 240;
          buf.data[0] = 'B' as u8;
          buf.data[1] = 'L' as u8;
          buf.data[2] = '\x00' as u8;
          for (buf_chunk, state_chunk) in buf.data[3..96]
            .chunks_mut(3)
            .take(31)
            .zip(lights.ground.chunks(3).rev())
          {
            buf_chunk[0] = state_chunk[1];
            buf_chunk[1] = state_chunk[0];
            buf_chunk[2] = state_chunk[2];
          }

          for (buf_chunks, state_chunk) in buf.data[96..240].chunks_mut(24).zip(
            lights
              .air_left
              .chunks(3)
              .rev()
              .chain(lights.air_right.chunks(3)),
          ) {
            for idx in 0..8 {
              buf_chunks[0 + idx * 3] = state_chunk[1];
              buf_chunks[1 + idx * 3] = state_chunk[0];
              buf_chunks[2 + idx * 3] = state_chunk[2];
            }
          }
        },
      ),
      HardwareSpec::Yuancon => Self::new(
        state.clone(),
        0x1973,
        0x2001,
        *disable_air,
        0x81,
        |buf, input| {
          if buf.len != 34 && buf.len != 35 {
            return;
          }

          input.ground.copy_from_slice(&buf.data[2..34]);
          for i in 0..6 {
            input.air[i ^ 1] = (buf.data[0] >> i) & 1;
          }
          for i in 0..3 {
            input.extra[2 - i] = (buf.data[1] >> i) & 1;
          }
        },
        vec![LedSpec::new(WriteType::Interrupt, 0x02)],
        |led_specs, lights| {
          let buf = led_specs[0].led_buf.borrow_mut();
          buf.len = 31 * 2;
          for (buf_chunk, state_chunk) in buf
            .data
            .chunks_mut(2)
            .take(31)
            .zip(lights.ground.chunks(3).rev())
          {
            buf_chunk[0] = (state_chunk[0] << 3 & 0xe0) | (state_chunk[2] >> 3);
            buf_chunk[1] = (state_chunk[1] & 0xf8) | (state_chunk[0] >> 5);
          }
        },
      ),
      HardwareSpec::Yubideck => Self::new(
        state.clone(),
        0x1973,
        0x2001,
        *disable_air,
        0x81, // Need to confirm
        |buf, input| {
          if buf.len != 45 && buf.len != 46 {
            return;
          }

          input.ground.copy_from_slice(&buf.data[2..34]);
          input.flip_vert();
          for i in 0..6 {
            input.air[i ^ 1] = (buf.data[0] >> i) & 1;
          }
          for i in 0..3 {
            input.extra[2 - i] = (buf.data[1] >> i) & 1;
          }
        },
        vec![LedSpec::new(WriteType::Interrupt, 0x02)],
        |led_specs, lights| {
          let buf = led_specs[0].led_buf.borrow_mut();

          buf.len = 62;

          let lights_nibbles: Vec<u8> = lights
            .ground
            .chunks(3)
            .rev()
            .flat_map(|x| x.iter().map(|y| *y >> 4))
            .chain([
              lights.air_left[3] >> 4,
              lights.air_left[4] >> 4,
              lights.air_left[5] >> 4,
            ])
            .collect();

          for (buf_chunk, state_chunk) in buf
            .data
            .chunks_mut(3)
            .take(16)
            .zip(lights_nibbles.chunks(6))
          {
            buf_chunk[0] = (state_chunk[0]) | (state_chunk[1] << 4);
            buf_chunk[1] = (state_chunk[2]) | (state_chunk[3] << 4);
            buf_chunk[2] = (state_chunk[4]) | (state_chunk[5] << 4);
          }
        },
      ),
      HardwareSpec::YubideckThree => Self::new(
        state.clone(),
        0x1973,
        0x2001,
        *disable_air,
        0x81, // Need to confirm
        |buf, input| {
          if buf.len != 45 && buf.len != 46 {
            return;
          }

          input.ground.copy_from_slice(&buf.data[2..34]);
          input.flip_vert();
          for i in 0..6 {
            input.air[i ^ 1] = (buf.data[0] >> i) & 1;
          }
          for i in 0..3 {
            input.extra[2 - i] = (buf.data[1] >> i) & 1;
          }
        },
        vec![
          LedSpec::new(WriteType::Interrupt, 0x02),
          LedSpec::new(WriteType::Interrupt, 0x02),
        ],
        |led_specs, lights| {
          if let [led_spec_a, led_spec_b] = led_specs.as_mut_slice() {
            let buf_a = &mut led_spec_a.led_buf;
            let buf_b = &mut led_spec_b.led_buf;

            buf_a.len = 61;
            buf_a.data[0] = 0;
            buf_b.len = 61;
            buf_b.data[0] = 1;

            for (buf_chunk, state_chunk) in buf_a.data[1..61]
              .chunks_mut(3)
              .zip(lights.ground.chunks(3).skip(11).take(20).rev())
            {
              buf_chunk[0] = state_chunk[0];
              buf_chunk[1] = state_chunk[1];
              buf_chunk[2] = state_chunk[2];
            }

            for (buf_chunk, state_chunk) in buf_b.data[1..34]
              .chunks_mut(3)
              .zip(lights.ground.chunks(3).take(11).rev())
            {
              buf_chunk[0] = state_chunk[0];
              buf_chunk[1] = state_chunk[1];
              buf_chunk[2] = state_chunk[2];
            }

            buf_b.data[34..37].copy_from_slice(&lights.air_left[3..6]);
            buf_b.data[37..40].copy_from_slice(&lights.air_right[3..6]);
          } else {
            panic!();
          }
        },
      ),
      HardwareSpec::HoriPad => Self::new(
        state.clone(),
        0x0f0d,
        0x0092,
        *disable_air,
        0x84,
        |buf, input| {
          if buf.len != 9 {
            return;
          }

          let bits: Vec<u8> = buf.data[1..8]
            .iter()
            .flat_map(|x| (0..8).map(move |i| ((x ^ 128) >> i) & 1))
            .collect();
          for i in 0..32 {
            input.ground[i] = bits[7 * 8 - 1 - i] * 255;
          }
          input.air.copy_from_slice(&bits[0..6]);

          input.extra[0] = 0;
          input.extra[1] = 0;
          input.extra[2] = 0;
        },
        vec![
          LedSpec::new(WriteType::Interrupt, 0x04),
          LedSpec::new(WriteType::Interrupt, 0x05),
          LedSpec::new(WriteType::Interrupt, 0x06),
          LedSpec::new(WriteType::Interrupt, 0x0b),
        ],
        |led_specs, lights| {
          if let [led_spec_ga, led_spec_gb, led_spec_air, led_spec_comp] = led_specs.as_mut_slice()
          {
            let buf_ga = &mut led_spec_ga.led_buf;
            let buf_gb = &mut led_spec_gb.led_buf;
            let buf_air = &mut led_spec_air.led_buf;
            let buf_comp = &mut led_spec_comp.led_buf;

            let light_rgb_buf: Vec<u8> = lights
              .ground
              .iter()
              .chain(lights.air_left.iter())
              .chain(lights.air_right.iter())
              .map(|x| *x)
              .collect();
            let light_brg_buf: Vec<u8> = light_rgb_buf
              .chunks(3)
              .map(|x| [x[2], x[0], x[1]])
              .flatten()
              .collect();

            let mut light_brg_buf_compress: Vec<u8> = vec![];
            light_brg_buf_compress.reserve(512);
            compress(&light_brg_buf, &mut light_brg_buf_compress);
            // info!("raw      {:?}", light_brg_buf);
            // info!("compress {:?}", light_brg_buf_compress);

            if light_brg_buf_compress.len() < 63 {
              buf_comp.data[0..light_brg_buf_compress.len()]
                .copy_from_slice(light_brg_buf_compress.as_slice());
            } else {
              buf_ga.data[0..48].copy_from_slice(&light_brg_buf[0..48]);
              buf_gb.data[0..45].copy_from_slice(&light_brg_buf[48..(48 + 45)]);
              buf_air.data[0..18].copy_from_slice(&light_brg_buf[(48 + 45)..(48 + 45 + 18)]);
            }
          }
        },
      ),
    }
  }

  fn get_handle(&mut self) -> Result<(), Box<dyn Error>> {
    info!("Device finding vid {} pid {}", self.vid, self.pid);
    let handle = rusb::open_device_with_vid_pid(self.vid, self.pid);
    if handle.is_none() {
      error!("Device not found");
      return Err(Box::new(ShimError));
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

impl ThreadJob for HidJob {
  fn setup(&mut self) -> bool {
    match self.get_handle() {
      Ok(_) => {
        info!("Device OK");
        true
      }
      Err(e) => {
        error!("Device setup failed: {}", e);
        false
      }
    }
  }

  fn tick(&mut self) -> bool {
    // Input loop
    let handle = self.handle.as_mut().unwrap();
    let mut work = false;

    {
      let res = handle
        .read_interrupt(self.read_endpoint, &mut self.read_buf.data, TIMEOUT)
        .map_err(|e| {
          // debug!("Device read error {}", &e);
          e
        })
        .unwrap_or(0);
      self.read_buf.len = res;
      // debug!("{:?}", self.read_buf.slice());
      // if self.read_buf.len != 0 {
      if (self.read_buf.len != 0) && (self.read_buf.slice() != self.last_read_buf.slice()) {
        work = true;
        let mut input_handle = self.state.input.lock();
        (self.read_callback)(&self.read_buf, input_handle.deref_mut());

        if self.disable_air {
          input_handle.air.fill(0);
        }
        swap(&mut self.read_buf, &mut self.last_read_buf);
      }
    }

    // Led loop
    {
      {
        let mut lights_handle = self.state.lights.lock();
        if lights_handle.dirty {
          (self.led_callback)(&mut self.led_specs, lights_handle.deref());
          lights_handle.dirty = false;
        }
      }

      for led_spec in self.led_specs.iter_mut() {
        if led_spec.led_buf.len != 0 {
          let res = (match led_spec.led_write_type {
            WriteType::Bulk => {
              handle.write_bulk(led_spec.led_endpoint, &led_spec.led_buf.slice(), TIMEOUT)
            }
            WriteType::Interrupt => {
              handle.write_interrupt(led_spec.led_endpoint, &led_spec.led_buf.slice(), TIMEOUT)
            }
          })
          .map_err(|e| {
            // debug!("Device write error {}", e);
            e
          })
          .unwrap_or(0);
          if res == led_spec.led_buf.len + 1 {
            work = true;
            led_spec.led_buf.len = 0;
          }
        }
      }
    }

    work
  }
}

impl Drop for HidJob {
  fn drop(&mut self) {
    if let Some(handle) = self.handle.as_mut() {
      handle.release_interface(0).ok();
    }
  }
}
