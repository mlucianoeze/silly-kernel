use std::{path::Path, process::Command, fs};
use clap::{Parser, Subcommand};
use anyhow::{Context, Result};

mod qemu;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    BuildKernel {
        #[arg(long)]
        release: bool,
    },
    BuildImage {
        #[arg(long)]
        release: bool,
    },
    Qemu {
        #[arg(long)]
        release: bool,
    },
    DebugQemu,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Cmd::BuildKernel { release } => build_kernel(release)?,
        Cmd::BuildImage { release } => build_image(release)?,
        Cmd::Qemu { release } => qemu_aarch64(release, false)?,
        Cmd::DebugQemu => qemu_aarch64(false, true)?,
    }

    Ok(())
}

fn build_kernel(release: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if release {
        cmd.arg("--release");
    }

    cmd.current_dir("kernel"); // workdir is `kernel` folder

    let status = cmd
        .status()
        .context("Failed to invoke `cargo build` for kernel")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    println!("\nSilly Kernel built successfully! :p\n");
    Ok(())
}

fn build_image(release: bool) -> Result<()> {
    build_kernel(release)?;
    
    let target_dir = Path::new("target");
    if !target_dir.exists() {
        return Err(anyhow::anyhow!("Target directory does not exist"));
    }
    let profile_dir = if release { "release" } else { "debug" };

    let mut found_binaries = Vec::new();

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let profile_path = path.join(profile_dir);
            if profile_path.exists() {
                let elf_path = profile_path.join("silly-kernel");
                if elf_path.exists() {
                    found_binaries.push((elf_path, profile_path));
                }
            }
        }
    }

    if found_binaries.is_empty() {
        return Err(anyhow::anyhow!("No silly-kernel binaries found in target directory"));
    }

    for (elf_path, output_dir) in found_binaries {
        let bin_path = output_dir.join("silly-kernel.bin");
        
        let status = Command::new("objcopy")
            .arg("-O")
            .arg("binary")
            .arg(&elf_path)
            .arg(&bin_path)
            .status()
            .with_context(|| {
                format!(
                    "Failed to invoke `objcopy` to convert {} to {}",
                    elf_path.display(), bin_path.display()
                )
            })?;

        if !status.success() {
            eprintln!("Failed to convert {} to {}", elf_path.display(), bin_path.display());
            std::process::exit(status.code().unwrap_or(1));
        }

        println!("Image binary generated: {}", bin_path.display());
    }

    println!("\nAll image binaries generated successfully!\n");
    Ok(())
}

fn qemu_aarch64(release: bool, debug_mode: bool) -> Result<()> {
    build_image(if debug_mode { false } else { release })?;
    
    let mut cmd = if debug_mode {
        qemu::aarch64::debug_qemu()
    } else {
        qemu::aarch64::qemu(release)
    };

    println!("Running QEMU...\nSerial monitor enabled, use Ctrl+A X to exit and Ctrl+A C to switch to monitor mode");
    let status = cmd.status()
        .context("Failed to run QEMU")?;
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    
    Ok(())
}


