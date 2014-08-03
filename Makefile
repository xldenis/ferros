NASM=nasm
QEMU=qemu-system-i386
RUSTC=rustc
LD=i386-elf-ld -m elf_i386
CLANG=clang
CARGO=cargo

all: floppy.img

.PHONY: clean run debug

run: floppy.img
	$(QEMU) -fda $<

loader.o: loader.asm
	$(NASM) -f elf32 -o $@ $<

main.o: main.rs
	$(RUSTC) -O --target i386-intel-linux --crate-type lib -o $*.bc --emit=bc $<
	$(CLANG) -ffreestanding -c $*.bc -o $@

floppy.img: linker.ld loader.o main.o
	$(LD) -o $@ -T $^

clean:
	rm -f *.bin *.img *.o *.bc

test: main.rs
	$(CARGO) -O --target i386-intel-linux --crate-type lib -o $*.bc --emit=bc --verbose --release
