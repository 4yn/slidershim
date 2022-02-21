fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/serial.cc")
        .file("src/serial_win.cc")
        .file("src/list_ports_win.cc")
        .file("src/wwserial.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/lib.rs");
    
    println!("cargo:rerun-if-changed=include/serial.h");
    println!("cargo:rerun-if-changed=include/serial_win.h");
    println!("cargo:rerun-if-changed=include/v8stdint.cc");

    println!("cargo:rerun-if-changed=src/serial.cc");
    println!("cargo:rerun-if-changed=src/serial_win.cc");
    println!("cargo:rerun-if-changed=src/list_ports_win.cc");

    println!("cargo:rerun-if-changed=src/wwserial.cc");
    println!("cargo:rerun-if-changed=include/wwserial.h");
}
