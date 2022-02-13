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
  controller_state::{FullState, LedState},
  shared::{utils::Buffer, voltex::VoltexState, worker::AsyncJob},
};

use super::config::{LedMode, ReactiveLayout};

pub struct LedJob {
  state: FullState,
  mode: LedMode,
  serial_port: Option<Box<dyn SerialPort>>,
  started: Instant,
  timer: Interval,
}

impl LedJob {
  pub fn new(state: &FullState, mode: &LedMode) -> Self {
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
    flat_controller_state: Option<&Vec<bool>>,
    serial_buffer: Option<&Buffer>,
    led_state: &mut LedState,
  ) {
    match self.mode {
      LedMode::Reactive { layout, .. } => {
        let flat_controller_state = flat_controller_state.unwrap();

        match layout {
          ReactiveLayout::Even { splits } => {
            let buttons_per_split = 32 / splits;

            let banks: Vec<bool> = flat_controller_state
              .chunks(32 / splits)
              .take(splits)
              .map(|x| x.contains(&true))
              .collect();

            for idx in 0..31 {
              led_state.paint(
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
            led_state.led_state.fill(0);

            // Fixed
            led_state.paint(3, &[10, 100, 180]);
            for idx in 0..5 {
              led_state.paint(7 + idx * 4, &[64, 64, 64]);
            }
            led_state.paint(27, &[180, 10, 110]);

            let voltex_state = VoltexState::from_flat(flat_controller_state);

            // Left laser
            for (idx, state) in voltex_state.laser[0..2].iter().enumerate() {
              if *state {
                led_state.paint(0 + idx * 4, &[70, 230, 250]);
                led_state.paint(1 + idx * 4, &[70, 230, 250]);
                led_state.paint(2 + idx * 4, &[70, 230, 250]);
              }
            }

            // Right laser
            for (idx, state) in voltex_state.laser[2..4].iter().enumerate() {
              if *state {
                led_state.paint(24 + idx * 4, &[250, 60, 200]);
                led_state.paint(25 + idx * 4, &[255, 60, 200]);
                led_state.paint(26 + idx * 4, &[255, 60, 200]);
              }
            }

            // Buttons
            for (idx, state) in voltex_state.bt.iter().enumerate() {
              if *state {
                led_state.paint(8 + idx * 4, &[255, 255, 255]);
                led_state.paint(10 + idx * 4, &[255, 255, 255]);
              }
            }

            // Fx
            for (idx, state) in voltex_state.fx.iter().enumerate() {
              if *state {
                led_state.paint(9 + idx * 8, &[250, 100, 30]);
                led_state.paint(11 + idx * 8, &[250, 100, 30]);
                led_state.paint(13 + idx * 8, &[250, 100, 30]);
              }
            }
          }
        }
      }
      LedMode::Attract => {
        let theta = self
          .started
          .elapsed()
          .div_duration_f64(Duration::from_secs(4))
          % 1.0;
        for idx in 0..31 {
          let slice_theta = (&theta + (idx as f64) / 32.0) % 1.0;
          let color = Srgb::from_color(Hsv::new(slice_theta * 360.0, 1.0, 1.0)).into_format::<u8>();
          led_state.paint(idx, &[color.red, color.green, color.blue]);
        }
      }
      LedMode::Serial { .. } => {
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
              led_state.paint(idx, &[(*buf_chunk)[1], (*buf_chunk)[2], (*buf_chunk)[0]]);
            }
            // println!("leds {:?}", led_state.led_state);
          }
        }
      }
      _ => panic!("Not implemented"),
    }

    led_state.dirty = true;
  }
}

#[async_trait]
impl AsyncJob for LedJob {
  async fn setup(&mut self) -> bool {
    match &self.mode {
      LedMode::Serial { port } => {
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
    let mut flat_controller_state: Option<Vec<bool>> = None;
    let mut serial_buffer: Option<Buffer> = None;

    // Do the IO here
    match self.mode {
      LedMode::Reactive { sensitivity, .. } => {
        let controller_state_handle = self.state.controller_state.lock();
        flat_controller_state = Some(controller_state_handle.to_flat(&sensitivity));
      }
      LedMode::Serial { .. } => {
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
      let mut led_state_handle = self.state.led_state.lock();
      self.calc_lights(
        flat_controller_state.as_ref(),
        serial_buffer.as_ref(),
        led_state_handle.deref_mut(),
      );
    }
    // thread::sleep(Duration::from_millis(30));
    // spin_sleep::sleep(Duration::from_micros(33333));
    self.timer.tick().await;

    true
  }
}
