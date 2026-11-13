use std::env;

/// Compiles the C++ host under `shim/` when the `cdm` feature is on. Without the feature the
/// crate is plain parsing and needs no C++ compiler at all.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if env::var_os("CARGO_FEATURE_CDM").is_none() {
        return;
    }
    for file in [
        "shim/shim.cc",
        "shim/cdm/content_decryption_module.h",
        "shim/cdm/content_decryption_module_export.h",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }

    let mut shim = cc::Build::new();
    // Nix enables _FORTIFY_SOURCE in debug builds too; glibc requires optimization.
    if env::var("OPT_LEVEL").as_deref() == Ok("0") {
        shim.opt_level(1);
    }
    shim.cpp(true)
        .std("c++14")
        .file("shim/shim.cc")
        .include("shim/cdm")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("/EHsc")
        .compile("cdmshim");

    // dlopen lives in libc on macOS and in libdl on Linux; Windows loads through kernel32,
    // which every Rust binary links already.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-link-lib=dylib=dl");
    }
}
