use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get target information from Cargo
    let _target = env::var("TARGET").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // Determine the cross-compiler based on target
    let compiler = match target_arch.as_str() {
        "aarch64" => "aarch64-none-elf-gcc",
        _ => panic!("Unsupported target architecture: {}", target_arch),
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let boot_o = out_dir.join("boot.o");

    let boot_s = PathBuf::from(format!("src/arch/{}/boot.S", target_arch));
    let linker_ld = PathBuf::from(format!("src/arch/{}/linker.ld", target_arch));
    
    // Tell cargo to rerun this build script if boot.S changes
    println!("cargo:rerun-if-changed={}", boot_s.display());
    println!("cargo:rerun-if-changed={}", linker_ld.display());

    // Compile boot.S to boot.o
    let status = Command::new(compiler)
        .args(&[
            "-Wall",
            "-O2", 
            "-ffreestanding",
            "-nostdlib",
            "-nostartfiles",
            "-c",
            boot_s.to_str().unwrap(),
            "-o",
            boot_o.to_str().unwrap(),
        ])
        .status()
        .expect(&format!("Failed to execute {}", compiler));

    if !status.success() {
        panic!("Failed to compile boot.S");
    }

    // Tell cargo to link the boot.o file directly
    println!("cargo:rustc-link-arg={}", boot_o.display());
}
