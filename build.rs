use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell cargo to rerun this build script if boot.S changes
    println!("cargo:rerun-if-changed=boot.S");
    println!("cargo:rerun-if-changed=linker.ld");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let boot_o = out_dir.join("boot.o");

    // Compile boot.S to boot.o
    let status = Command::new("aarch64-none-elf-gcc")
        .args(&[
            "-Wall",
            "-O2", 
            "-ffreestanding",
            "-nostdlib",
            "-nostartfiles",
            "-c",
            "boot.S",
            "-o",
            boot_o.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute aarch64-none-elf-gcc");

    if !status.success() {
        panic!("Failed to compile boot.S");
    }

    // Tell cargo to link the boot.o file directly
    println!("cargo:rustc-link-arg={}", boot_o.display());
}
