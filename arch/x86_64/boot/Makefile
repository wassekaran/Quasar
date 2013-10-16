TARGET = loader.o
SRC = $(TARGET:.o=.s)

all: $(TARGET)

$(TARGET): $(SRC)
	$(AS) $(ASFLAGS) -o $@ $^

clean:
	rm -rf *.o
	rm -rf $(TARGET)

