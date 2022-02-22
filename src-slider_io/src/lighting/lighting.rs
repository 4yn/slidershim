use async_trait::async_trait;
use log::{error, info};
use palette::{FromColor, Hsv, Srgb};
use serialport::{ClearBuffer, SerialPort};
use std::{
  ops::DerefMut,
  time::{Duration, Instant},
};
use tokio::time::{interval, Interval};

use crate::{
  shared::{utils::Buffer, voltex::VoltexState, worker::AsyncJob},
  state::{SliderLights, SliderState},
};

use super::config::{LightsMode, ReactiveLayout};

pub struct LightsJob {
  state: SliderState,
  mode: LightsMode,
  serial_port: Option<Box<dyn SerialPort>>,
  started: Instant,
  timer: Interval,
}

impl LightsJob {
  pub fn new(state: &SliderState, mode: &LightsMode) -> Self {
    Self {
      state: state.clone(),
      mode: mode.clone(),
      serial_port: None,
      started: Instant::now(),
      timer: interval(Duration::from_micros(33333)),
    }
  }

  fn calc_lights(
    &self,
    flat_input: Option<&Vec<bool>>,
    serial_buffer: Option<&Buffer>,
    lights: &mut SliderLights,
  ) {
    match self.mode {
      LightsMode::Reactive { layout, .. } => {
        let flat_input = flat_input.unwrap();

        match layout {
          ReactiveLayout::Even { splits } => {
            let buttons_per_split = 32 / splits;

            let banks: Vec<bool> = flat_input
              .chunks(32 / splits)
              .take(splits)
              .map(|x| x.contains(&true))
              .collect();

            for idx in 0..31 {
              lights.paint(
                idx,
                match (idx + 1) % buttons_per_split {
                  0 => &[255, 0, 255],
                  _ => match banks[idx / buttons_per_split] {
                    true => &[255, 0, 255],
                    false => &[255, 255, 0],
                  },
                },
              );
            }
          }
          ReactiveLayout::Voltex => {
            lights.ground.fill(0);

            // Fixed
            lights.paint(3, &[10, 100, 180]);
            for idx in 0..5 {
              lights.paint(7 + idx * 4, &[64, 64, 64]);
            }
            lights.paint(27, &[180, 10, 110]);

            let voltex_input = VoltexState::from_flat(flat_input);

            // Left laser
            for (idx, state) in voltex_input.laser[0..2].iter().enumerate() {
              if *state {
                lights.paint(0 + idx * 4, &[70, 230, 250]);
                lights.paint(1 + idx * 4, &[70, 230, 250]);
                lights.paint(2 + idx * 4, &[70, 230, 250]);
              }
            }

            // Right laser
            for (idx, state) in voltex_input.laser[2..4].iter().enumerate() {
              if *state {
                lights.paint(24 + idx * 4, &[250, 60, 200]);
                lights.paint(25 + idx * 4, &[255, 60, 200]);
                lights.paint(26 + idx * 4, &[255, 60, 200]);
              }
            }

            // Buttons
            for (idx, state) in voltex_input.bt.iter().enumerate() {
              if *state {
                lights.paint(8 + idx * 4, &[255, 255, 255]);
                lights.paint(10 + idx * 4, &[255, 255, 255]);
              }
            }

            // Fx
            for (idx, state) in voltex_input.fx.iter().enumerate() {
              if *state {
                lights.paint(9 + idx * 8, &[250, 100, 30]);
                lights.paint(11 + idx * 8, &[250, 100, 30]);
                lights.paint(13 + idx * 8, &[250, 100, 30]);
              }
            }
          }
          ReactiveLayout::Rainbow => {
            let banks: Vec<bool> = flat_input
              .chunks(2)
              .take(16)
              .map(|x| x.contains(&true))
              .collect();
            let theta = self
              .started
              .elapsed()
              .div_duration_f64(Duration::from_secs(4))
              % 1.0;
            for idx in 0..31 {
              let slice_theta = (&theta + (idx as f64) / 32.0) % 1.0;
              let color = Srgb::from_color(Hsv::new(
                slice_theta * 360.0,
                match idx % 2 {
                  0 => match banks[idx / 2] {
                    true => 0.2,
                    false => 1.0,
                  },
                  1 => 1.0,
                  _ => unreachable!(),
                },
                1.0,
              ))
              .into_format::<u8>();
              lights.paint(idx, &[color.red, color.green, color.blue]);
            }
          }
        }
      }
      LightsMode::Attract => {
        let theta = self
          .started
          .elapsed()
          .div_duration_f64(Duration::from_secs(4))
          % 1.0;
        for idx in 0..31 {
          let slice_theta = (&theta + (idx as f64) / 32.0) % 1.0;
          let color = Srgb::from_color(Hsv::new(slice_theta * 360.0, 1.0, 1.0)).into_format::<u8>();
          lights.paint(idx, &[color.red, color.green, color.blue]);
        }
      }
      LightsMode::Serial { .. } => {
        // https://github.com/jmontineri/OpeNITHM/blob/89e9a43f7484e8949cd31bbff79c32f21ea3ec1d/Firmware/OpeNITHM/SerialProcessor.h
        // https://github.com/jmontineri/OpeNITHM/blob/89e9a43f7484e8949cd31bbff79c32f21ea3ec1d/Firmware/OpeNITHM/SerialProcessor.cpp
        // https://github.com/jmontineri/OpeNITHM/blob/89e9a43f7484e8949cd31bbff79c32f21ea3ec1d/Firmware/OpeNITHM/SerialLeds.h
        // https://github.com/jmontineri/OpeNITHM/blob/89e9a43f7484e8949cd31bbff79c32f21ea3ec1d/Firmware/OpeNITHM/SerialLeds.cpp
        if let Some(serial_buffer) = serial_buffer {
          // println!("buffer {:?}", serial_buffer.data);
          if serial_buffer.data[0] == 0xaa && serial_buffer.data[1] == 0xaa {
            for (idx, buf_chunk) in serial_buffer.data[2..95]
              .chunks(3)
              .take(31)
              .rev()
              .enumerate()
            {
              lights.paint(idx, &[(*buf_chunk)[1], (*buf_chunk)[2], (*buf_chunk)[0]]);
            }
          }
        }
      }
      _ => panic!("Not implemented"),
    }

    lights.dirty = true;
  }
}

#[async_trait]
impl AsyncJob for LightsJob {
  async fn setup(&mut self) -> bool {
    match &self.mode {
      LightsMode::Serial { port } => {
        info!(
          "Serial port for led opening at {} {:?}",
          port.as_str(),
          115200
        );
        self.serial_port = match serialport::new(port, 115200).open() {
          Ok(s) => {
            info!("Serial port opened");
            Some(s)
          }
          Err(e) => {
            error!("Serial port could not open: {}", e);
            None
          }
        };

        self.serial_port.is_some()
      }
      _ => true,
    }
  }

  async fn tick(&mut self) -> bool {
    let mut flat_input: Option<Vec<bool>> = None;
    let mut serial_buffer: Option<Buffer> = None;

    // Do the IO here
    match self.mode {
      LightsMode::Reactive { sensitivity, .. } => {
        let input_handle = self.state.input.lock();
        flat_input = Some(input_handle.to_flat(&sensitivity));
      }
      LightsMode::Serial { .. } => {
        if let Some(serial_port) = self.serial_port.as_mut() {
          let mut serial_data_avail = serial_port.bytes_to_read().unwrap_or(0);
          if serial_data_avail >= 100 {
            if serial_data_avail % 100 == 0 {
              let mut serial_buffer_working = Buffer::new();
              serial_port
                .as_mut()
                .read_exact(&mut serial_buffer_working.data[..100])
                .ok()
                .unwrap();
              serial_data_avail -= 100;
              serial_buffer = Some(serial_buffer_working);
            }

            if serial_data_avail > 0 {
              serial_port.clear(ClearBuffer::All).unwrap();
            }
          }
        }
      }
      _ => {}
    }

    // Then calculate and transfer
    {
      let mut lights_handle = self.state.lights.lock();
      self.calc_lights(
        flat_input.as_ref(),
        serial_buffer.as_ref(),
        lights_handle.deref_mut(),
      );
    }
    self.timer.tick().await;

    true
  }
}
