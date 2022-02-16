use std::{mem, ptr};
use winapi::um::{commapi::*, fileapi::*, minwinbase::*, synchapi::*, winbase::*, winnt::*};

fn main() {
  unsafe {
    let mut port: Vec<u16> = vec![];
    port.extend("COM4".encode_utf16());
    port.push(0);

    let handle = CreateFileW(
      port.as_ptr(),
      GENERIC_READ | GENERIC_WRITE,
      0,
      ptr::null_mut(),
      OPEN_EXISTING,
      FILE_ATTRIBUTE_NORMAL | FILE_FLAG_OVERLAPPED,
      0 as HANDLE,
    );

    let mut overlapped_read: OVERLAPPED = mem::zeroed();
    overlapped_read.hEvent = CreateEventW(ptr::null_mut(), 1, 0, ptr::null_mut());
    let mut overlapped_write: OVERLAPPED = mem::zeroed();
    overlapped_write.hEvent = CreateEventW(ptr::null_mut(), 0, 0, ptr::null_mut());

    SetupComm(handle, 4096, 4096);

    let mut timeouts: COMMTIMEOUTS = mem::zeroed();
    GetCommTimeouts(handle, &mut timeouts);

    SetCommMask(handle, 0x80); // EV_ERR

    let mut dcb: DCB = mem::zeroed();
    GetCommState(handle, &mut dcb);

    dcb.BaudRate = 115200;
    dcb.ByteSize = 8;
    dcb.Parity = NOPARITY;
    dcb.set_fParity(0);
    dcb.StopBits = ONESTOPBIT;
    dcb.set_fBinary(1);

    PurgeComm(
      handle,
      PURGE_TXCLEAR | PURGE_TXABORT | PURGE_RXCLEAR | PURGE_RXABORT,
    );
  }
}
