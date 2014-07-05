NASM=nasm
QEMU=qemu-system-i386
RUSTC=rustc
LD=ld -melf_i386

all: floppy.img

run: floppy.img
	$(QEMU) -fda $<

floppy.img: loader.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &> /dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &> /dev/null

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.o: main.rs
	$(RUSTC) -O --target i386-intel-linux --crate-type lib -o $@ --emit obj $<

main.bin: linker.ld main.o
	$(LD) -o $@ -T $^

clean:
	rm -f *.bin *.img