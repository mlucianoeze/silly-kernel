OUTPUT_ARCH(aarch64)
ENTRY(_head)

SECTIONS
{
    . = 0x40000000;                 /* Default load address for many systems */
    
    .text : {
        *(.text.boot)               /* Header and entry code first */
        *(.text*)
    }
    
    .rodata : ALIGN(4096) {
        *(.rodata*)
    }
    
    .data : ALIGN(4096) {
        *(.data*)
    }
    
    .bss : ALIGN(4096) {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }
    
    _end = .;
    
    . = ALIGN(4096);
    
    .stack : {
        . = . + 0x10000;            /* 64KB stack */
        stack_top = .;
    }
}
