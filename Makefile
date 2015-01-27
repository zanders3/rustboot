LD=bin/i386-elf-ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run

librlibc.rlib:
	$(RUSTC) -O --target i686-unknown-linux-gnu -L ./bin -C relocation-model=static --crate-type lib -o $@ rlibc.rs

librlibc.o:
	$(RUSTC) -O --target i686-unknown-linux-gnu -L ./bin -C relocation-model=static --crate-type lib -o $@ --emit obj rlibc.rs

main.o: librlibc.rlib
	$(RUSTC) -O --target i686-unknown-linux-gnu -L ./ -L ./bin -C relocation-model=static --crate-type lib -o $@ --emit obj main.rs

.asm.o:
	$(NASM) -f elf32 -o $@ $<

floppy.img: loader.bin main.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld main.o librlibc.o
	$(LD) -m elf_i386 -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img *.rlib
