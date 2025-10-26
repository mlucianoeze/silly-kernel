#![no_std]
#![no_main]
use core::arch::asm;

use devtree::DevTree;

mod machine;

fn main() {
    machine::enable_neon();
    let devtree = machine::get_devtree();
    if devtree.is_none() {
        panic!("Failed to get DevTree");
    }
    let devtree: DevTree = devtree.unwrap();
    let root = devtree.root();
    
    // Iterate over root node properties
    for prop in root.properties() {
        let _name = prop.name();
        let _value = prop.value();
    }
    
    let mut count: usize = 0;
    // Iterate over root node children
    for child in root.children() {
        let _child_name = child.name();
        count += 1;
    }

    panic!("Count of children: {}", count);
    // ... continue working with the DTB
}


#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {  asm!("mov x11, #0x69") };
    main();
    unsafe { asm!("wfi") };
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { 
        asm!("mov x12, #0x6969");
        asm!("wfi");
    }
    loop {}
}
