HOST_ARCH = $(shell uname -m)

ifeq ("x"$(ARCH), "x")
	ARCH = $(HOST_ARCH)
endif

TARGET = $(ARCH)-elf-linux
TARGET_BIN = quasar.img
TARGET_EFI = quasar.efi
LOADER = arch/$(ARCH)/boot/loader.o

EFI_LD = x86_64-w64-mingw32-ld
EFI_LDFLAGS = --oformat pei-x86-64 --subsystem 10 -pie -e efi_main

KERNEL_MAIN = entry.rs
KERNEL_RUST_SRC = $(wildcard *.rs)
KERNEL_RUST_ARCH_SRC = $(wildcard arch/$(ARCH)/*.rs)
KERNEL_ASM_SRC = $(wildcard arch/$(ARCH)/*.s)

KERNEL_RUST_OBJ = $(KERNEL_MAIN:.rs=.o)
KERNEL_ASM_OBJ = $(KERNEL_ASM_SRC:.s=.o)
KERNEL_OBJS = $(KERNEL_RUST_OBJ) $(KERNEL_ASM_OBJ)

# Build tools and options

export AS = nasm
export ASFLAGS = -felf64 

export LD = x86_64-elf-ld 
export LDFLAGS = -nodefaultlibs -Tlinker.ld 

export RUSTC = rustc
export RUSTFLAGS = --lib -O # --target $(TARGET)

all: $(TARGET_BIN)

uefi: $(TARGET_EFI)

$(TARGET_EFI): $(KERNEL_OBJS)
	$(EFI_LD) $(EFI_LDFLAGS) -o $@ $^

$(TARGET_BIN): $(LOADER) $(KERNEL_OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $^

$(KERNEL_RUST_OBJ): $(KERNEL_MAIN) $(KERNEL_RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) -c $(KERNEL_MAIN) -o $@

$(LOADER):
	make -C arch/$(ARCH)/boot/

clean:
	rm -rf *.o
	rm -rf $(TARGET_BIN)
	rm -rf arch/$(ARCH)/*.o
	make -C arch/$(ARCH)/boot/ clean

