global long_mode_start

extern main

section .text
bits 64

long_mode_start:
   call main
   hlt
