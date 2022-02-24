use log::{error, info, warn};
use std::{
  collections::VecDeque,
  time::{Duration, Instant},
}; // thread::sleep, time::Duration
use wwserial::WwSerial;

use crate::{shared::worker::ThreadJob, state::SliderState};

/*
Init packet
0xff 0x10 0x00 0xf1

Report of all touch sliders at 16 pressure
0xff 0x01 0x20 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0x10 0xe0

Report of all touch sliders at 0-31 pressure
0xff 0x01 0x20 0x00 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0a 0x0b 0x0c 0x0d 0x0e 0x0f 0x10 0x11 0x12 0x13 0x14 0x15 0x16 0x17 0x18 0x19 0x1a 0x1b 0x1c 0x1d 0x1e 0x1f 0x0f
*/

#[derive(Debug)]
struct DivaPacket {
  command: u8,
  len: u8,
  data: Vec<u8>,
  checksum: u8,
  raw: Option<Vec<u8>>,
}

impl DivaPacket {
  fn new() -> Self {
    Self {
      command: 0,
      len: 0,
      data: Vec::with_capacity(256),
      checksum: 0,
      raw: None,
    }
  }

  fn from_bytes(command: u8, data: &[u8]) -> Self {
    let checksum = 0xffu64
      + (command as u64)
      + (data.len() as u64)
      + data.iter().map(|x| (*x) as u64).sum::<u64>();
    let checksum = ((0x100 - (checksum & 0xff)) & 0xff) as u8;

    Self {
      command,
      len: data.len() as u8,
      data: data.iter().copied().collect(),
      checksum: checksum,
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

  fn serialize(&mut self) -> &Vec<u8> {
    let mut raw: Vec<u8> = Vec::with_capacity(512);

    raw.push(0xff);
    Self::push_raw_escaped(self.command, &mut raw);
    Self::push_raw_escaped(self.len, &mut raw);
    for i in &self.data {
      Self::push_raw_escaped(*i, &mut raw);
    }
    Self::push_raw_escaped(self.checksum, &mut raw);

    // null pad?
    // raw.push(0);

    // debug!("Diva serializing {}", raw.len());
    self.raw = Some(raw);
    // debug!("Diva serializing {:?}", &self.raw);
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
  checksum: u64,
  escape: u8,
  len: u8,
  packet: DivaPacket,
}

impl DivaDeserializer {
  fn new() -> Self {
    Self {
      state: DivaDeserializerState::Done,
      checksum: 0,
      escape: 0,
      len: 0,
      packet: DivaPacket::new(),
    }
  }

  fn deserialize(&mut self, data: &[u8], out: &mut VecDeque<DivaPacket>) {
    // debug!("Diva deserializing {} {:?}", data.len(), data);
    for c in data {
      match c {
        0xff => {
          self.packet = DivaPacket::new();
          self.checksum = 0xff;
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

          self.checksum += c as u64;
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
              self.packet.checksum = c;
              debug_assert!(self.checksum & 0xff == 0);
              // println!("Packet complete {} {}", self.checksum, c);
              if self.checksum & 0xff == 0 {
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
  ReadLoop,
}

pub struct DivaSliderJob {
  state: SliderState,
  port: String,
  brightness: u8,
  read_buf: Vec<u8>,
  in_packets: VecDeque<DivaPacket>,
  out_packets: VecDeque<DivaPacket>,
  deserializer: DivaDeserializer,
  serial_port: Option<WwSerial>,
  bootstrap: DivaSliderBootstrap,
  last_lights: Instant,
}

impl DivaSliderJob {
  pub fn new(state: &SliderState, port: &String, brightness: u8) -> Self {
    Self {
      state: state.clone(),
      port: port.clone(),
      brightness,
      read_buf: Vec::with_capacity(1024),
      in_packets: VecDeque::with_capacity(100),
      out_packets: VecDeque::with_capacity(100),
      deserializer: DivaDeserializer::new(),
      serial_port: None,
      bootstrap: DivaSliderBootstrap::Init,
      last_lights: Instant::now(),
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

    let serial_port = WwSerial::new(self.port.clone(), 115200, 5, 0, false);
    if !serial_port.check() {
      error!("Cannot open serial port at {}", self.port.as_str());
      return false;
    }
    self.serial_port = Some(serial_port);
    true
  }

  fn tick(&mut self) -> bool {
    let mut work = false;

    let serial_port = self.serial_port.as_mut().unwrap();

    self.read_buf.clear();
    let read_amount = serial_port.read(&mut self.read_buf) as usize;
    if read_amount > 0 {
      // debug!("Serial read {} bytes", read_amount);
      self
        .deserializer
        .deserialize(&self.read_buf[0..read_amount], &mut self.in_packets);
    }

    match self.bootstrap {
      DivaSliderBootstrap::Init => {
        info!("Diva sending init");
        let reset_packet = DivaPacket::from_bytes(0x10, &[]);
        self.out_packets.push_back(reset_packet);
        self.bootstrap = DivaSliderBootstrap::AwaitReset;
      }
      DivaSliderBootstrap::AwaitReset => {
        while let Some(ack_packet) = self.in_packets.pop_front() {
          if ack_packet.command == 0x10 && ack_packet.len == 0x00 && ack_packet.checksum == 0xf1 {
            info!(
              "Diva ack init {:#4x} {:?}",
              ack_packet.command, ack_packet.data
            );

            info!("Diva sending info");
            let info_packet = DivaPacket::from_bytes(0xf0, &[]);
            self.out_packets.push_back(info_packet);
            self.bootstrap = DivaSliderBootstrap::AwaitInfo;
            break;
          } else {
            warn!(
              "Unexpected packet {:#4x} {:?}",
              ack_packet.command, ack_packet.data
            );
          }
        }
      }
      DivaSliderBootstrap::AwaitInfo => {
        if let Some(ack_packet) = self.in_packets.pop_front() {
          info!(
            "Diva ack info {:#4x} {:?}",
            ack_packet.command, ack_packet.data
          );

          info!("Diva sending start");
          let start_packet = DivaPacket::from_bytes(0x03, &[]);
          self.out_packets.push_back(start_packet);
          self.bootstrap = DivaSliderBootstrap::ReadLoop;
          self.last_lights = Instant::now();
        }
      }
      DivaSliderBootstrap::ReadLoop => {
        while let Some(data_packet) = self.in_packets.pop_front() {
          if data_packet.command == 0x01 && data_packet.len == 32 {
            let mut input_handle = self.state.input.lock();
            input_handle
              .ground
              .copy_from_slice(&data_packet.data[0..32]);
            input_handle.flip_all();
            work = true;
          }
        }

        let mut send_lights = false;
        let mut lights_buf = [0; 94];
        {
          let mut lights_handle = self.state.lights.lock();
          // Send leds at least once a second to keep alive
          if lights_handle.dirty || self.last_lights.elapsed() > Duration::from_millis(1000) {
            send_lights = true;
            lights_buf[0] = self.brightness;
            for (buf_chunk, state_chunk) in lights_buf[1..94]
              .chunks_mut(3)
              .take(31)
              .zip(lights_handle.ground.chunks(3).rev())
            {
              buf_chunk[0] = state_chunk[2];
              buf_chunk[1] = state_chunk[0];
              buf_chunk[2] = state_chunk[1];
            }
            lights_handle.dirty = false;
          }
        }

        if send_lights {
          self.last_lights = Instant::now();
          let lights_packet = DivaPacket::from_bytes(0x02, &lights_buf);
          self.out_packets.push_back(lights_packet);
        }
      }
    };

    // sleep(Duration::from_millis(3));
    while let Some(mut packet) = self.out_packets.pop_front() {
      work = true;
      let data = packet.serialize();
      let bytes_written = serial_port.write(data);
      if bytes_written == 0 {
        warn!("Serial write timeout");
      }
      serial_port.flush();
      // debug!("Serial write {}/{}", bytes_written, data.len());
    }

    // sleep(Duration::from_millis(3));

    // TODO: async worker?
    // sleep(Duration::from_millis(10));

    work
  }
}

impl Drop for DivaSliderJob {
  fn drop(&mut self) {
    match self.bootstrap {
      DivaSliderBootstrap::ReadLoop => {
        let serial_port = self.serial_port.as_mut().unwrap();
        let mut stop_packet = DivaPacket::from_bytes(0x04, &[]);
        serial_port.write(stop_packet.serialize());
      }
      _ => {}
    };
    info!("Diva serial port closed");
  }
}
