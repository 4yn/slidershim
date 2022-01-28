use std::{
  error,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
  time::Duration,
};

use rusb::{self, UsbContext};

const timeout: Duration = Duration::from_millis(20);

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

pub fn poll_controller(
  vid: u16,
  pid: u16,
  read_callback: impl Fn(&Buffer) -> (),
  write_callback: impl Fn(&mut Buffer) -> (),
  // write_buf: Arc<Mutex<Buffer>>,
  stop: &AtomicBool,
) -> Result<(), Box<dyn error::Error>> {
  // println!("Getting context");
  // let mut context = rusb::Context::new().unwrap();
  // println!("Getting devices");
  // let devices: Vec<rusb::Device<rusb::Context>> = context
  //   .devices()
  //   .unwrap()
  //   .iter()
  //   .filter(|d| {
  //     d.device_descriptor()
  //       .map(|d| d.vendor_id() == vid && d.product_id() == pid)
  //       .unwrap_or(false)
  //   })
  //   .collect();
  // println!("Found {:?}", devices);
  // let mut handle = devices[0].open().unwrap();

  let mut handle = rusb::open_device_with_vid_pid(vid, pid).unwrap();
  println!("Found device {:?}", handle);
  // .ok_or("Cannot find usb device".to_string())?;

  // let device = handle.device();
  if handle.kernel_driver_active(0).unwrap_or(false) {
    println!("Disabling kernel driver");
    handle.detach_kernel_driver(0)?;
  }

  println!("Kernel driver OK");
  handle.set_active_configuration(1)?;
  println!("Configuration OK");
  handle.claim_interface(0)?;
  println!("Interface OK");

  let mut in_buf = Buffer::new();
  let mut led_buf = Buffer::new();
  loop {
    {
      // Read loop

      let res = handle
        .read_interrupt(0x81, &mut in_buf.data, timeout)
        .unwrap_or(0);

      in_buf.len = res;

      // println!("Read {:?}", res);
      // println!("Data {:?}", in_buf.data);

      if in_buf.len != 0 {
        read_callback(&in_buf);
      }
    }

    {
      // Write loop
      write_callback(&mut led_buf);
      if led_buf.len != 0 {
        let res = handle
          .write_interrupt(0x02, led_buf.slice(), timeout)
          .unwrap_or(0);

        // println!(
        //   "Sent {:?} {:?} {:?}",
        //   led_buf.len,
        //   res,
        //   led_buf.slice().len()
        // );
        if res == led_buf.len + 1 {
          led_buf.len = 0;
        }
      }
    }

    {
      if stop.load(Ordering::SeqCst) {
        break;
      }
    }
  }

  println!("HID thread stopped");

  handle.release_interface(0)?;

  Ok(())
}
