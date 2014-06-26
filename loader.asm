use16                 ; BIOS is in 16-bit real mode
org 0x7c00            ; bootloader loaded to offset 0x7c00

mov ax, 0x7c00
mov ds, ax            ; set data segment to 0x7c00

cli                   ; clear interrupts while stack is being setup
mov ax, 0x9000 
mov ss, ax            ; start stack at 0x9000
mov sp, 0xffff
sti                   ; let the interrupts resume

times 510-($-$$) db 0 ; pad with 0s
db 0x55               ; place 0xAA55 at offset 510
db 0xaa