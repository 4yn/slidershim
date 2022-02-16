extern crate slider_io;

use std::{thread::sleep, time::Duration};

fn main() {
  let mut sp = serialport::new("COM4", 115200).open().unwrap();
  sp.write_request_to_send(true).unwrap();
  sp.write_data_terminal_ready(true).unwrap();

  sleep(Duration::from_millis(100));
  println!("tx");
  let res = sp.write(&[0xff, 0x01, 0x00, 0xf1]).unwrap();
  println!("tx {}", res);

  loop {
    sleep(Duration::from_millis(100));
    println!("rx");
    let mut buf = [0u8; 4];
    let res = sp.read(&mut buf).unwrap();
    println!("rx {} {:?}", res, buf);
  }
}

// use serial2::SerialPort;

// fn main() {
//   let sp = SerialPort::open("COM4", 115200).unwrap();
//   println!("Tx");
//   let res = sp.write("data".as_bytes()).unwrap();
//   println!("Tx {}", res);
//   loop {
//     println!("Rx");
//     let mut buf = [0u8; 5];
//     let res = sp.read(&mut buf).unwrap();
//     // let res = sp.read(&mut buf).unwrap_or_else(|e| {
//     //   println!("Err {}", e);
//     //   0
//     // });
//     println!("Rx {} {:?}", res, buf);
//   }

// let mut sp = serialport::new("COM4", 115200).open().unwrap();
// println!("Tx");
// let res = sp.write("data".as_bytes()).unwrap();
// println!("Tx {}", res);
// loop {
//   println!("Rx");
//   let mut buf = [0u8; 5];
//   let res = sp.read(&mut buf).unwrap();
//   println!("Rx {} {:?}", res, buf);
// }
// }

// use serialport::SerialPort;
// use std::time::Duration;
// use tokio::{
//   io::{self, AsyncReadExt, AsyncWriteExt},
//   time::sleep,
// };
// use tokio_serial::SerialPortBuilderExt;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//   let mut sp = tokio_serial::new("COM4", 115200)
//     .open_native_async()
//     .unwrap();

//   sp.write_request_to_send(true).unwrap();
//   sp.write_data_terminal_ready(true).unwrap();
//   // sp.set_timeout(Duration::from_millis(1000))?;
//   let (mut rx_port, mut tx_port) = tokio::io::split(sp);

//   // tx_port.write(&[41, 42, 43, 44]).await?;
//   // let mut buf = [0u8; 10];
//   // let res = rx_port.read(&mut buf).await?;
//   // println!("{}, {:?}", res, buf);

//   loop {
//     let mut buf = [0u8; 10];
//     let res = rx_port.read(&mut buf).await?;
//     println!("{}, {:?}", res, buf);
//   }

//   // let mut serial_reader =
//   //   tokio_util::codec::FramedRead::new(rx_port,
//   // tokio_util::codec::BytesCodec::new()); let serial_sink =
//   //   tokio_util::codec::FramedWrite::new(tx_port,
//   // tokio_util::codec::BytesCodec::new());

//   // println!("Tx");
//   // let res = tx.write("data".as_bytes()).await?;
//   // println!("Sent {}", res);
//   // sleep(Duration::from_millis(1000)).await;
//   // loop {
//   //   let mut buffer = [0u8; 1];
//   //   println!("Rx");
//   //   let res = rx.read(&mut buffer).await;
//   //   println!("{:?} {:?}", res, buffer);
//   // }

//   Ok(())
// }
