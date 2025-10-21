# silly-kernel

[README en Español](README.md)

So I'm on vacation with my new MacBook Air M4 and wanted to experiment with ARM architecture. In that moment of curiosity and madness that characterizes me so much, I came up with the idea of tackling this new project.

For this occasion, I don't have that much experience with bare-metal development, but nothing that reading an obscene amount of documentation and trying to understand ARM64 flows couldn't solve. After all, I'm comfortable with very low-level projects, so this was going to be fun anyway.

The plan to start with this kernel is quite simple: implement the Linux boot protocol to be able to use existing bootloaders, then interpret device trees, have extremely simple drivers for detected devices like displays or serial monitors (so I can write a hello world or at least draw something cute on screen), and then, only if I don't get bored before, move on to more advanced things like managing virtual memory, having process abstraction, trying to run simple programs, filesystems (even if it's just FAT32) and the like.

The thing about this code is that, like many other projects born from curiosity, I'm doing it without knowing (completely) how a kernel works. I never saw the code of one from scratch. The plan is to do it according to my own interpretation of how I think one would be.

Like everything in this life, I'm doing this just for the fun of it, because I love this stuff and want to learn from it. This project will stay alive as long as I keep enjoying it.

## Requirements

You need Rust stable with target `aarch64-unknown-none`, and ideally a GCC toolchain for the target `aarch64-none-elf-gcc`. To boot the kernel, it's recommended to boot the kernel with QEMU (especially if the only drivers I'm going to write are for it). All of this is already configured in a [Nix devShell](flake.nix) to have a ready-to-use environment.

Being a flake, in case you want to use Nix, you can start a devShell in the following way:

```bash
nix develop
```

## Building

You can use `cargo` without problem, it has [everything it needs](build.rs) to generate a kernel in ELF format.

```bash
cargo build
```

## Running

The kernel can be booted by converting it to a binary image and running it with QEMU:

```bash
aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/debug/silly-kernel target/aarch64-unknown-none/debug/silly-kernel.bin
qemu-system-aarch64 -M virt -cpu cortex-a57 -kernel target/aarch64-unknown-none/debug/silly-kernel.bin -display none -serial mon:stdio -gdb tcp::1234 -S
```

## Useful Links

* https://krinkinmu.github.io/2020/12/26/position-independent-executable.html
