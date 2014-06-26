use16 ; BIOS is in 16-bit real mode
org 0x7c00 ; bootloader loaded to offset 0x7c00

times 510-($-$$) db 0 ; pad with 0s
db 0x55 ; place 0xAA55 at offset 510
db 0xaa