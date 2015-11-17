; registers
; 8 bit  ah al bh bl ch cl dh dl
; 16 bit ax bx cx dx
; 32 bit eax ebx ecx edx

global start
extern main

section .text
bits 32

start:
    mov dword [0xb8000], 0x2f4b2f4f
    hlt



gdtr:
    dw (gdt_end - gdt) + 1 ; size
    dd gdt                 ; offset

idtr:                      ; lol ignore all errors
    dw 0
    dd 0

gdt:
                           ; null entry
    dq 0
                           ; code entry
    dw 0xffff              ; limit 0:15
    dw 0x0000              ; base 0:15
    db 0x00                ; base 16:23
    db 0b10011010          ; access byte - code
    db 0x4f                ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00                ; base 24:31
                           ; data entry
    dw 0xffff              ; limit 0:15
    dw 0x0000              ; base 0:15
    db 0x00                ; base 16:23
    db 0b10010010          ; access byte - data
    db 0x4f                ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00                ; base 24:31
gdt_end:

times 510-($-$$) db 0      ; pad with 0s
db 0x55                    ; place 0xAA55 at offset 510
db 0xaa
