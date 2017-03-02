### Build

To build you will need the following tools

* A recent version of the Rust compiler. It can be found at [http://github.com/mozilla/rust/](http://github.com/mozilla/rust/). Build hasn't been tested with versions older than 0.8
* A version of GNU binutils that generate code for the x86_64 architecture (x86_64-elf-linux-ld, especially)
* NASM, the Netwide assembler. Can be found at [http://www.nasm.us/](http://www.nasm.us/)

### Testing

To test Quasar, you will have to use Grub2. The easiest way is to use grub-mkrescue to generate a very simple ISO image. 
You will need the grub2 tools, that can be found at [http://ftp.gnu.org/gnu/grub](http://ftp.gnu.org/gnu/grub)

First create the directory structure of the ISO image :

```shell
mkdir -p iso/boot/grub
```

Create a simple config file for Grub 2 :

```shell
cat > iso/boot/grub/grub.cfg << EOF
set timeout=3
set default=0

menuentry "My Os" {
    multiboot2 /boot/quasar.img
    boot
}
EOF
```

Copy the kernel image to the location where Grub will be looking for it :

```shell
cp quasar.img iso/boot
```

Generate the iso :

```shell
grub-mkrescue -o test.iso iso/
```

In order to boot it inside Qemu, run :

```shell
qemu-system-x86_64 -monitor stdio test.iso
```
