; registers
; 8 bit  ah al bh bl ch cl dh dl
; 16 bit ax bx cx dx
; 32 bit eax ebx ecx edx

use16                 ; BIOS is in 16-bit real mode
org 0x7c00            ; bootloader loaded to offset 0x7c00

xor ax, ax
mov ds, ax

mov si, msg

call print

cli                   ; clear interrupts while stack is being setup
mov ax, 0x9000 
mov ss, ax            ; start stack at 0x9000
mov sp, 0xffff
sti                   ; let the interrupts resume

print:
  lodsb
  or al, al
  jz done
  mov ah, 0eh
  mov bx, 0007
  int 0x10
  jmp print
done:
  ret

msg db 'Hello World', 13, 10, 0

times 510-($-$$) db 0 ; pad with 0s
db 0x55               ; place 0xAA55 at offset 510
db 0xaa