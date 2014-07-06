NASM=nasm
QEMU=qemu-system-i386
RUSTC=rustc
LD=i386-elf-ld -m elf_i386

all: floppy.img

run: floppy.img
	$(QEMU) -fda $<

loader.o: loader.asm
	$(NASM) -f elf32 -o $@ $<

main.o: main.rs
	$(RUSTC) -O --target i386-intel-linux --crate-type lib -o $@ --emit obj $<

floppy.img: linker.ld loader.o main.o
	$(LD) -o $@ -T $^

clean:
	rm -f *.bin *.img *.o
