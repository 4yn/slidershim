#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("lzfx/include/lzfxbridge.h");

        fn compress(data: &Vec<u8>, out: &mut Vec<u8>) -> u32;

        fn decompress(data: &Vec<u8>, out: &mut Vec<u8>) -> u32;
    }
}

pub use ffi::compress;
pub use ffi::decompress;
