Quasar is an experiment for an OS kernel written in Rust. Quasar is still at the
very beginning of its development but aims for a full-64 bits micro-kernel
architecture with shared-memory IPC and in-user-space UNIX compatibility. For
the moment, the following has been implemented:

* The kernel is linked as a multiboot2-compliant ELF binary.
* Boot in long (64 bits) mode with paging and GDT setup, with the kernel mapped
  in higher-half then jump into Rust code.
* 80x25 character-mode screen management.
* Interrupt Descriptor Table.

### Build

To build you will need the following tools:

* A recent nightly version of the [Rust programming language].
* A version of GNU binutils that generate code for the x86_64 architecture.
* [NASM], the Netwide assembler.

[Rust programming language]: http://rust-lang.org/
[NASM]: http://www.nasm.us/

Run the following to build Quasar:

```
autoreconf
./configure
make
```

### Testing

The Makefile exports a `make iso` target to build a minimal iso image based on
GRUB2, as well as a `make run` target to run the generated image with QEMU.

Those targets add the following dependencies:

* The [GRUB2] tools, especially you will need `grub-mkrescue`.
* The `xorriso` tool to deal with iso images. `grub-mkrescue` needs it to be
in path, or the generated iso image will be invalid.
* QEMU, for the `make run` target. You can also try to run the image in another
x86_64 emulator, but QEMU is recommanded if you want to hack on Quasar because
it provides debugging facilities.

[GRUB2]: http://ftp.gnu.org/gnu/grub

```
make run    # Or just `make iso` if you want to use another virtual machine.
```
