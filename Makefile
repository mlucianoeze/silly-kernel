CROSS_COMPILE = aarch64-none-elf-
CC = $(CROSS_COMPILE)gcc
LD = $(CROSS_COMPILE)ld
OBJCOPY = $(CROSS_COMPILE)objcopy

CFLAGS = -Wall -O2 -ffreestanding -nostdlib -nostartfiles
LDFLAGS = -T linker.ld

all: Image

boot.o: boot.S
	$(CC) $(CFLAGS) -c boot.S -o boot.o

# kernel.o: kernel.c
#	$(CC) $(CFLAGS) -c kernel.c -o kernel.o

# kernel.elf: boot.o kernel.o
#	$(LD) $(LDFLAGS) boot.o kernel.o -o kernel.elf

# Image: kernel.elf
#	$(OBJCOPY) -O binary kernel.elf Image

kernel.elf: boot.o
	$(LD) $(LDFLAGS) boot.o -o kernel.elf

Image: kernel.elf
	$(OBJCOPY) -O binary kernel.elf Image

clean:
	rm -f *.o kernel.elf Image

qemu: Image
	qemu-system-aarch64 -M virt -cpu cortex-a57 \
		-kernel Image -nographic
