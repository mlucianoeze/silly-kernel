#![no_std]
#![no_main]
use core::arch::asm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe { 
        asm!("mov x11, #0x69");
        asm!("wfi");
    };
    loop {}
}
