fn main() {
    println!("cargo:rerun-if-changed=include/interception.h");
    println!("cargo:rerun-if-changed=src/interception.c");
    cc::Build::new()
        .define("INTERCEPTION_STATIC", None)
        .file("src/interception.c")
        .include("include/")
        .compile("interception");

    println!("cargo:rustc-link-lib=static=interception");
}
