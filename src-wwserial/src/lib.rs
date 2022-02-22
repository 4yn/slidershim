#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wwserial/include/wwserial.h");

        type CxxSerial;

        fn new_cxx_serial(
            port: String,
            baud: u32,
            read_timeout: u32,
            write_timeout: u32,
            hardware: bool,
        ) -> UniquePtr<CxxSerial>;

        fn write(self: &CxxSerial, data: &Vec<u8>) -> u32;

        fn read(self: &CxxSerial, data: &mut Vec<u8>) -> u32;

        fn flush(self: &CxxSerial);

        fn check(self: &CxxSerial) -> bool;
    }
}

pub use ffi::new_cxx_serial;

pub struct WwSerial {
    inner: cxx::UniquePtr<ffi::CxxSerial>,
}

unsafe impl Send for WwSerial {}

impl WwSerial {
    pub fn new(
        port: String,
        baud: u32,
        read_timeout: u32,
        write_timeout: u32,
        hardware: bool,
    ) -> Self {
        Self {
            inner: new_cxx_serial(port, baud, read_timeout, write_timeout, hardware),
        }
    }

    // #[inline(always)]
    pub fn write(&self, data: &Vec<u8>) -> u32 {
        self.inner.write(data)
    }

    // #[inline(always)]
    pub fn read(&self, data: &mut Vec<u8>) -> u32 {
        self.inner.read(data)
    }

    // #[inline(always)]
    pub fn flush(&self) {
        self.inner.flush()
    }

    // #[inline(always)]
    pub fn check(&self) -> bool {
        self.inner.check()
    }
}
