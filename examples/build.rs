use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Reference: https://docs.rs/riscv-rt/latest/riscv_rt/

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy `memory.x` to the output directory
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    // Add search path for linker scripts
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Tell the cargo to rebuild when `memory.x` or this script is updated
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.x");
}
