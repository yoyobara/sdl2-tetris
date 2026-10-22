use std::{env, fs, path::PathBuf};

// Links against the vendored SDL2 import library on Windows and copies
// SDL2.dll next to the built executable so `cargo run` works out of the box.
fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = manifest_dir.join("vendor/SDL2/lib/x64");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rerun-if-changed={}", lib_dir.display());

    // OUT_DIR is target/<profile>/build/<pkg>-<hash>/out; the exe lives in target/<profile>.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile_dir = out_dir.ancestors().nth(3).unwrap();
    fs::copy(lib_dir.join("SDL2.dll"), profile_dir.join("SDL2.dll"))
        .expect("failed to copy SDL2.dll");
}
