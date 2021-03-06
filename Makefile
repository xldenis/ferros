TARGET 			?= x86_64-unknown-linux-gnu

NASM 				?= nasm
QEMU 				?= qemu-system-x86_64
LD 					= x86_64-elf-ld
CLANG 			?= clang
CARGO_CLEAN ?= cargo clean
GRUB_MK     ?= grub-mkrescue

arch 				?= x86_64

TARGETDIR 	?= target/$(TARGET)/debug
CARGOFLAGS  ?= --target=$(TARGET)
RUSTCFLAGS  ?= -Z no-landing-pads
kernel 			:= build/kernel-$(arch).bin
iso    			:= build/os-$(arch).iso

linker_script 				:= src/arch/$(arch)/linker.ld
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(assembly_source_files))
grub_cfg := src/arch/$(arch)/grub.cfg
rust_os := $(TARGETDIR)/libferros.a

all: $(kernel)

.PHONY: clean run debug

run: iso
	$(QEMU) -drive format=raw,file=$(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp -R $(kernel) build/isofiles/boot/kernel.bin
	cp -R $(grub_cfg) build/isofiles/boot/grub
	$(GRUB_MK) -o $(iso) build/isofiles 2> /dev/null
	rm -r build/isofiles

$(kernel): $(linker_script) $(assembly_object_files) cargo $(rust_os)
	$(LD) -n --gc-sections -o $@ -T $(linker_script) $(assembly_object_files) $(rust_os)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	$(NASM) -f elf64 $< -o $@

cargo:
	@cargo rustc $(CARGOFLAGS) -- $(RUSTCFLAGS)

clean:
	rm -rf $(TARGETDIR)
	rm -rf build/
	$(CARGO_CLEAN)
