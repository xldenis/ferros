; registers
; 8 bit  ah al bh bl ch cl dh dl
; 16 bit ax bx cx dx
; 32 bit eax ebx ecx edx

use16 

global __morestack
global abort
global memcmp
global memcpy
global malloc
global free
global start

extern main

start: 
xor ax, ax
mov ds, ax
mov es, ax

cli                        ; clear interrupts while stack is being setup
mov ax, 0x9000 
mov ss, ax                 ; start stack at 0x9000
mov sp, 0xffff
sti                        ; let the interrupts resume

mov ah, 2             ; disk read
mov al, 64            ; num blocks
mov ch, 0             ; cylinder
mov cl, 2             ; sector
mov dh, 0             ; disk head
mov bx, 0x7e00        ; put kernel at 0x7e00
int 0x13              ; reset disk controller

cli
lgdt [gdtr]                ; global descriptor table (register)
lidt [idtr]                ; interrupt descriptor table (register)

mov eax, cr0
or eax, 1
mov cr0, eax            ; switch to protected mode

jmp 0x08:pmode

pmode:
use32
; load all the other segments with 32-bit segment 2 (data)
mov eax, 2 << 3
mov ds, eax
mov es, eax
mov fs, eax
mov gs, eax
mov ss, eax
; set up aligned stack bottom
mov eax, 0x7bff
mov esp, eax
; enable SSE instructions
; mov eax, cr4
; or eax, 512
; mov cr4, eax
; mov dword[gs:0x30], 
mov edi, 0xb8000
mov ecx, 80*25*2
mov al, 0
rep stosb
mov [gs:0x30], dword 0
call main

abort:
__morestack:
memcmp:
memcpy:
malloc:
free:
  jmp $


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
