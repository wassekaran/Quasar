HOST_ARCH = $(shell uname -m)

ifeq ("x"$(ARCH), "x")
	ARCH = $(HOST_ARCH)
endif

TARGET = $(ARCH)-elf-linux
TARGET_BIN = quasar.img
LOADER = arch/$(ARCH)/boot/loader.o
KERNEL_SRC = entry.rs
KERNEL_OBJS = $(KERNEL_SRC:.rs=.o) arch/$(ARCH)/handlers.o

# Build tools and options

export AS = nasm
export ASFLAGS = -felf64 

export LD = x86_64-elf-ld 
export LDFLAGS = -nodefaultlibs -Tlinker.ld 

export RUSTC = rustc
export RUSTFLAGS = --lib -O --target $(TARGET)

all: $(TARGET_BIN)

$(TARGET_BIN): $(LOADER) $(KERNEL_OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

arch/$(ARCH)/handlers.o: arch/$(ARCH)/handlers.s
	$(AS) $(ASFLAGS) -o $@ $^

%.o: %.rs
	$(RUSTC) $(RUSTFLAGS) -c $^ -o $@

$(LOADER):
	make -C arch/$(ARCH)/boot/

clean:
	rm -rf *.o
	rm -rf $(TARGET_BIN)
	rm -rf arch/$(ARCH)/*.o
	make -C arch/$(ARCH)/boot/ clean

