ENTRY(main)
OUTPUT_FORMAT(elf32-i386)

MEMORY {
  ram : org 0x7e00, l = 12k
}

SECTIONS {
  . = 0x7e00

  .text : {
    *(.text)
  } > ram
  /DISCARD/ : {
    *(.comment)
    *(.eh_frame)
    *(.rel.eh_frame)
  }
}