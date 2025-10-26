use std::process::Command;

const QEMU_AARCH64_MACHINE_ARGS: &[&str] = &[
    "-M", "virt",
    "-cpu", "cortex-a57",
    "-display", "none",
    "-serial", "mon:stdio"
];

pub fn qemu(release: bool) -> Command {
    let mut cmd = Command::new("qemu-system-aarch64");
    cmd.args(QEMU_AARCH64_MACHINE_ARGS);

    let profile_dir = if release { "release" } else { "debug" };
    let kernel_path = format!("target/aarch64-unknown-none/{}/silly-kernel.bin", profile_dir);
    cmd.args(&["-kernel", &kernel_path]);

    cmd
}

pub fn debug_qemu() -> Command {
    let mut cmd = qemu(false);
    cmd.args(&["-gdb", "tcp::1234", "-S"]);
    cmd
}
