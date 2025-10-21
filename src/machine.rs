use core::arch::asm;
use devtree::DevTree;

pub fn enable_neon() {
    unsafe {
        asm!("mov x24, x0");
        asm!("mrs x0, CPACR_EL1");
        asm!("orr x0, x0, #(3 << 20)");
        asm!("msr CPACR_EL1, x0");
        asm!("isb");
        asm!("mov x0, x24");
    }
}

pub fn get_devtree() -> Option<DevTree> {
    let mut dtb_ptr: *const u8;
    unsafe { 
        asm!("mov {}, x0", out(reg) dtb_ptr) 
    };
    
    DevTree::new(dtb_ptr)
}