NASM 				?= nasm
QEMU 				?= qemu-system-i386
RUSTC 			?= rustc
LD 					= i386-elf-ld -m elf_i386
CLANG 			?= clang
CARGO 			?= cargo build
CARGO_CLEAN ?= cargo clean
TARGET 			?= i686-unknown-linux-gnu

BDIR 				?= build/x86
TARGETDIR 	?= target/$(TARGET)/release

arch 				?= x86

RUSTCFLAGS  ?= --target=$(TARGET) 
RUSTCFLAGS2 ?= -O --emit=obj -Z no-landing-pads --crate-name main -C relocation-model=static

all: floppy.img

.PHONY: clean run debug

run: floppy.img
	$(QEMU) -fda $<

loader.o: loader.asm
	$(NASM) -f elf32 -o $@ $<

floppy.img: linker.ld loader.o $(BDIR)/main.o
	$(LD) -o $@ -T $^

clean:
	rm -f *.bin *.img *.o *.bc
	rm -f build/$(arch)/*.o
	$(CARGO_CLEAN)

$(BDIR)/main.o: src/*.rs
	- $(CARGO) $(RUSTCFLAGS) --verbose --release
	$(RUSTC) src/lib.rs $(RUSTCFLAGS) $(RUSTCFLAGS2) --out-dir $(BDIR) -L $(TARGETDIR)/deps
