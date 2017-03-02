TARGET = quasar.img
LOADER = arch/x86_64/boot/loader.o
KERNEL_SRC = entry.rs
KERNEL_OBJS = $(KERNEL_SRC:.rs=.o)

# Build tools and options

export AS = nasm
export ASFLAGS = -felf64 

export LD = x86_64-elf-ld 
export LDFLAGS = -nodefaultlibs -Tlinker.ld 

export RUSTC = rustc
export RUSTFLAGS = --lib -O --target x86_64-elf-linux

all: $(TARGET)

$(TARGET): $(LOADER) $(KERNEL_OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

%.o: %.rs
	$(RUSTC) $(RUSTFLAGS) -c $^

$(LOADER):
	make -C arch/x86_64/boot/

clean:
	rm -rf *.o
	rm -rf $(TARGET)
	make -C arch/x86_64/boot/ clean

