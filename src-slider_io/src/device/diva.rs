use log::{error, info};
use serialport::{COMPort, SerialPort};
use std::{
  collections::VecDeque,
  io::{Read, Write},
  num::Wrapping,
  thread::sleep,
  time::Duration,
};

use crate::{
  shared::{serial::ReadWriteTimeout, worker::ThreadJob},
  state::SliderState,
};

struct DivaPacket {
  command: u8,
  len: u8,
  data: Vec<u8>,
  checksum: Wrapping<u8>,
  raw: Option<Vec<u8>>,
}

impl DivaPacket {
  fn new() -> Self {
    Self {
      command: 0,
      len: 0,
      data: Vec::with_capacity(256),
      checksum: Wrapping(0),
      raw: None,
    }
  }

  fn from_bytes(command: u8, data: &[u8]) -> Self {
    Self {
      command,
      len: data.len() as u8,
      data: data.iter().copied().collect(),
      checksum: Wrapping(0),
      raw: None,
    }
  }

  fn push_raw_escaped(byte: u8, raw: &mut Vec<u8>) {
    match byte {
      0xfd => {
        raw.push(0xfd);
        raw.push(0xfc);
      }
      0xff => {
        raw.push(0xfd);
        raw.push(0xfe);
      }
      _ => {
        raw.push(byte);
      }
    }
  }

  fn serialize(&mut self) -> &[u8] {
    let mut raw: Vec<u8> = Vec::with_capacity(512);
    let mut checksum = Wrapping(0);

    raw.push(0xff);
    checksum += Wrapping(0xffu8);
    Self::push_raw_escaped(self.command, &mut raw);
    checksum += Wrapping(self.command);
    Self::push_raw_escaped(self.len, &mut raw);
    checksum += Wrapping(self.len);
    for i in &self.data {
      Self::push_raw_escaped(*i, &mut raw);
      checksum += Wrapping(*i);
    }

    checksum = -checksum;
    Self::push_raw_escaped(checksum.0, &mut raw);

    self.raw = Some(raw);
    return self.raw.as_ref().unwrap();
  }
}

enum DivaDeserializerState {
  ExpectCommand,
  ExpectLen,
  ExpectData,
  ExpectChecksum,
  Done,
}

struct DivaDeserializer {
  state: DivaDeserializerState,
  escape: u8,
  len: u8,
  packet: DivaPacket,
}

impl DivaDeserializer {
  fn new() -> Self {
    Self {
      state: DivaDeserializerState::Done,
      escape: 1,
      len: 0,
      packet: DivaPacket::new(),
    }
  }

  fn deserialize(&mut self, data: &[u8], out: &mut VecDeque<DivaPacket>) {
    // println!("Found data");
    for c in data {
      match c {
        0xff => {
          self.packet = DivaPacket::new();
          self.packet.checksum = Wrapping(0xff);
          self.state = DivaDeserializerState::ExpectCommand;
          self.escape = 0;

          // println!("{} open", c);
        }
        0xfd => {
          self.escape = 1;
          // println!("esc {}", c);
        }
        c => {
          let c = c + self.escape;
          self.escape = 0;

          self.packet.checksum += Wrapping(c);
          match self.state {
            DivaDeserializerState::ExpectCommand => {
              self.packet.command = c;
              self.state = DivaDeserializerState::ExpectLen;

              // println!("cmd {}", c);
            }
            DivaDeserializerState::ExpectLen => {
              self.len = c;
              self.packet.len = c;
              self.state = match c {
                0 => DivaDeserializerState::ExpectChecksum,
                _ => DivaDeserializerState::ExpectData,
              };
              // println!("len {}", c);
            }
            DivaDeserializerState::ExpectData => {
              self.packet.data.push(c);
              self.len -= 1;

              if self.len == 0 {
                self.state = DivaDeserializerState::ExpectChecksum;
              }

              // println!("data {}", c);
            }
            DivaDeserializerState::ExpectChecksum => {
              // println!("checksum {} {:?}", c, self.packet.checksum);
              debug_assert!(self.packet.checksum == Wrapping(0));
              if self.packet.checksum == Wrapping(0) {
                out.push_back(DivaPacket::new());
                std::mem::swap(&mut self.packet, out.back_mut().unwrap());
              }
              self.state = DivaDeserializerState::Done;
            }
            _ => {}
          }
        }
      }
    }
  }
}

enum DivaSliderBootstrap {
  Init,
  AwaitReset,
  AwaitInfo,
  AwaitStart,
  ReadLoop,
}

pub struct DivaSliderJob {
  state: SliderState,
  port: String,
  packets: VecDeque<DivaPacket>,
  deserializer: DivaDeserializer,
  serial_port: Option<COMPort>,
  bootstrap: DivaSliderBootstrap,
}

impl DivaSliderJob {
  pub fn new(state: &SliderState, port: &String) -> Self {
    Self {
      state: state.clone(),
      port: port.clone(),
      packets: VecDeque::with_capacity(100),
      deserializer: DivaDeserializer::new(),
      serial_port: None,
      bootstrap: DivaSliderBootstrap::Init,
    }
  }
}

impl ThreadJob for DivaSliderJob {
  fn setup(&mut self) -> bool {
    info!(
      "Serial port for diva slider opening at {} {:?}",
      self.port.as_str(),
      115200
    );
    match serialport::new(&self.port, 152000).open_native() {
      Ok(serial_port) => {
        info!("Serial port opened");
        serial_port
          .set_read_write_timeout(Duration::from_millis(3))
          .ok();
        self.serial_port = Some(serial_port);
        true
      }
      Err(e) => {
        error!("Serial port could not open: {}", e);
        false
      }
    }
  }

  fn tick(&mut self) -> bool {
    let mut work = false;

    let serial_port = self.serial_port.as_mut().unwrap();

    let bytes_avail = serial_port.bytes_to_read().unwrap_or(0);
    if bytes_avail > 0 {
      let mut read_buf = vec![0 as u8; bytes_avail as usize];
      serial_port.read(&mut read_buf).ok();
      self.deserializer.deserialize(&read_buf, &mut self.packets);
      work = true;
    }

    match self.bootstrap {
      DivaSliderBootstrap::Init => {
        info!("Diva sending init");
        let mut reset_packet = DivaPacket::from_bytes(0x10, &[]);
        match serial_port.write(reset_packet.serialize()) {
          Ok(_) => {
            info!("Diva sent init");

            self.bootstrap = DivaSliderBootstrap::AwaitReset;
            work = true;
          }
          Err(e) => {
            error!("Diva send init error {}", e);
          }
        }

        // Wait for flush
        sleep(Duration::from_millis(100));
      }
      DivaSliderBootstrap::AwaitReset => {
        while self.packets.len() > 1 {
          self.packets.pop_front();
        }
        if let Some(ack_packet) = self.packets.pop_front() {
          info!(
            "Diva ack reset {:?} {:?}",
            ack_packet.command, ack_packet.data
          );

          let mut info_packet = DivaPacket::from_bytes(0xf0, &[]);

          match serial_port.write(info_packet.serialize()) {
            Ok(_) => {
              info!("Diva sent info");

              self.bootstrap = DivaSliderBootstrap::AwaitInfo;
              work = true;
            }
            Err(e) => {
              error!("Diva send info error {}", e);
            }
          }
        }
      }
      DivaSliderBootstrap::AwaitInfo => {
        if let Some(ack_packet) = self.packets.pop_front() {
          info!(
            "Diva ack info {:?} {:?}",
            ack_packet.command, ack_packet.data
          );

          let mut start_packet = DivaPacket::from_bytes(0x03, &[]);

          match serial_port.write(start_packet.serialize()) {
            Ok(_) => {
              info!("Diva sent start");

              self.bootstrap = DivaSliderBootstrap::AwaitStart;
              work = true;
            }
            Err(e) => {
              error!("Diva send start error {}", e);
            }
          }
        }
      }
      DivaSliderBootstrap::AwaitStart => {
        if let Some(ack_packet) = self.packets.pop_front() {
          info!(
            "Diva ack start {:?} {:?}",
            ack_packet.command, ack_packet.data
          );

          self.bootstrap = DivaSliderBootstrap::ReadLoop;
          work = true;
        }
      }
      DivaSliderBootstrap::ReadLoop => {
        while let Some(data_packet) = self.packets.pop_front() {
          if data_packet.command == 0x01 && data_packet.len == 32 {
            let mut input_handle = self.state.input.lock();
            input_handle
              .ground
              .copy_from_slice(&data_packet.data[0..32]);
            work = true;
          }
        }

        let mut send_lights = false;
        let mut lights_buf = [0; 97];
        {
          let mut lights_handle = self.state.lights.lock();
          if lights_handle.dirty {
            send_lights = true;
            lights_buf[0] = 0x3f;
            lights_buf[1..97].copy_from_slice(&lights_handle.ground[0..96]);
            lights_handle.dirty = false;
          }
        }

        if send_lights {
          let mut lights_packet = DivaPacket::from_bytes(0x02, &lights_buf);
          serial_port.write(lights_packet.serialize()).ok();
        }
      }
    };

    // TODO: async worker?
    sleep(Duration::from_millis(10));

    work
  }
}

impl Drop for DivaSliderJob {
  fn drop(&mut self) {
    match self.bootstrap {
      DivaSliderBootstrap::AwaitStart | DivaSliderBootstrap::ReadLoop => {
        let serial_port = self.serial_port.as_mut().unwrap();
        let mut stop_packet = DivaPacket::from_bytes(0x04, &[]);
        serial_port.write(stop_packet.serialize()).ok();
      }
      _ => {}
    };
    info!("Diva serial port closed");
    // println!("Diva slider dropped");
  }
}
