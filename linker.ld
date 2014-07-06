ENTRY(start)
OUTPUT_FORMAT(binary)

MEMORY {
  ram : org = 0x7e00, l = 34k
}

SECTIONS {
  . = 0x7c00;

  .text : {
    loader.o(.text)
    *(.text)
    *(.gnu.linkonce.t*)
  } >ram
  
  /DISCARD/ : {
    *(.comment)
    *(.eh_frame)
    *(.rel.eh_frame)
  }
}