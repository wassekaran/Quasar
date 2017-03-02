#![crate_name = "quasar"]
#![crate_type = "dylib"]

#![feature(globs)]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(core_slice_ext)]
#![no_std]

//#![deny(warnings)]

pub use native::arch;

//pub mod runtime;

mod native;
pub mod rlibc;

#[no_mangle]
pub unsafe fn kmain () {
    arch::io::console::println("QUASAR");
    arch::idt::setup();
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    arch::io::console::println("kernel panic");
    loop {}
}
