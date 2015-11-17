ENTRY(start)

SECTIONS {
    . = 1M;
    .boot : {
        KEEP(*(.multiboot_header))
    }
    .text : {
        *(.text)
        *(.gnu.linkonce.t*)
    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
