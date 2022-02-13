use serialport::COMPort;
use std::{os::windows::prelude::AsRawHandle, time::Duration};

use winapi::{
  shared::minwindef::DWORD,
  um::{commapi::SetCommTimeouts, winbase::COMMTIMEOUTS},
};

pub trait ReadWriteTimeout {
  fn set_read_write_timeout(&self, timeout: Duration) -> Result<(), ()>;
}

impl ReadWriteTimeout for COMPort {
  fn set_read_write_timeout(&self, timeout: Duration) -> Result<(), ()> {
    let milliseconds = timeout.as_secs() * 1000 + timeout.subsec_nanos() as u64 / 1_000_000;

    let mut timeouts = COMMTIMEOUTS {
      ReadIntervalTimeout: 0,
      ReadTotalTimeoutMultiplier: 0,
      ReadTotalTimeoutConstant: milliseconds as DWORD,
      WriteTotalTimeoutMultiplier: 0,
      WriteTotalTimeoutConstant: milliseconds as DWORD,
    };

    if unsafe { SetCommTimeouts(self.as_raw_handle(), &mut timeouts) } == 0 {
      return Err(());
    }

    Ok(())
  }
}
