SRC = loader.s
TARGET = $(SRC:.s=.o)

all: $(TARGET)

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $^

clean:
	rm -rf *.o
	rm -rf $(TARGET)

