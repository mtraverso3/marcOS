all: os-image.bin

# link with -ffreestanding to avoid linking with stdlib
# -Ttext 0x1000 to tell the linker to start at 0x1000, this is where the bootloader will load the kernel
kernel.bin: kernel_entry.o kernel.o
	i686-elf-ld -o $@ -Ttext 0x1000 $^ --oformat binary

kernel_entry.o: boot/kernel_entry.asm
	nasm $< -f elf -o $@

kernel.o: kernel/kernel.c
	i686-elf-gcc -ffreestanding -c $< -o $@

# Rule to disassemble the kernel for debugging
kernel.dis: kernel.bin
	ndisasm -b 32 $< > $@

bootsect.bin: boot/bootsect.asm
	nasm $< -f bin -o $@

os-image.bin: bootsect.bin kernel.bin
	cat $^ > $@

run: os-image.bin
	qemu-system-i386 -fda $<

clean:
	rm *.bin *.o *.dis
