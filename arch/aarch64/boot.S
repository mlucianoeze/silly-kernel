.section ".text.boot"

// ARM64 image header (64 bytes)
.global _head
_head:
    b       _start                  // code0: branch to actual start
    .long   0                       // code1: reserved/NOP
    .quad   0                       // text_offset: 0 = can load anywhere
    .quad   _end - _head            // image_size: size of kernel image  
    .quad   0x0a                    // flags: see below
    .quad   0                       // res2
    .quad   0                       // res3
    .quad   0                       // res4  
    .long   0x644d5241              // magic: ARM\x64 (little-endian)
    .long   0                       // res5: PE/COFF offset

_start:
    // Save DTB pointer from x0
    mov     x20, x0
    
    // Ensure we're at EL1 (or handle EL2)
    mrs     x0, CurrentEL
    and     x0, x0, #12             // Extract EL
    cmp     x0, #4                  // EL1?
    b.eq    el1_entry
    cmp     x0, #8                  // EL2?
    b.eq    el2_entry
    
    // Unsupported exception level
    b       .
    
el2_entry:
    // Drop from EL2 to EL1
    mov     x0, #0x3c5              // EL1h (EL1 with SP_EL1)
    msr     spsr_el2, x0
    adr     x0, el1_entry
    msr     elr_el2, x0
    eret                            // Exception return to EL1

el1_entry:
    // Now we're in EL1
    
    // Disable MMU, caches
    mrs     x0, sctlr_el1
    bic     x0, x0, #1              // Clear M bit (MMU)
    bic     x0, x0, #(1<<2)         // Clear C bit (data cache)
    bic     x0, x0, #(1<<12)        // Clear I bit (instruction cache)
    msr     sctlr_el1, x0
    isb
    
    // Set up stack
    ldr     x0, =stack_top
    mov     sp, x0
    
    // Clear BSS
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
bss_clear_loop:
    cmp     x0, x1
    b.ge    bss_done
    str     xzr, [x0], #8
    b       bss_clear_loop
    
bss_done:
    // Restore DTB pointer and jump to C
    mov     x0, x20
    mov     x10, #0x69
    bl      kernel_main
    
halt:
    wfi
    b       halt
