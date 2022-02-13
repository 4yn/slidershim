// use serialport::SerialPort;
// use std::io::{BufRead, BufReader};

// struct ArcadeSlider {
//   serial_port: BufReader<Box<dyn SerialPort>>,
// }

// impl ArcadeSlider {
//   fn new() -> Self {
//     let serial_port = serialport::new("COM1", 152000).open().unwrap();
//     let serial_port_buf = BufReader::new(serial_port);
//     Self {
//       serial_port: serial_port_buf,
//     }
//   }

//   fn recv(&mut self) {
//     let mut consumed = 0;
//     {
//       let d = self.serial_port.fill_buf().unwrap();

//       let mut packets = vec![];
//       let mut packet = vec![];

//       let mut bytes_taken = 0;
//       let mut in_escape = 0;
//       let mut checksum = 0;
//       for b in d.iter() {
//         bytes_taken += 1;

//         match b {
//           0xff => {
//             consumed += bytes_taken;
//             bytes_taken = 0;
//             in_escape = 0;
//           }
//           0xfd => {
//             in_escape = 1;
//           }
//           _ => {
//             let b = b + in_escape;
//             in_escape = 0;
//             packet.push(b);
//           }
//         }
//       }
//     }
//     self.serial_port.consume(consumed);
//   }

//   fn send(&mut self) {}
// }
