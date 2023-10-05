fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/lzfx.cc")
        .file("src/lzfxbridge.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lzfx.cc");
    println!("cargo:rerun-if-changed=src/lzfxbridge.cc");
    println!("cargo:rerun-if-changed=include/lzfx.h");
    println!("cargo:rerun-if-changed=include/lzfxbridge.h");
}
