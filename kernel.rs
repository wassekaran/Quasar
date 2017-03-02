#![crate_name = "quasar"]
#![crate_type = "dylib"]

#![feature(globs)]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]

#![allow(ctypes)]
#![deny(managed_heap_memory)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_qualification)]
#![deny(unnecessary_typecast)]
#![deny(unused_result)]
#![deny(warnings)]

#![allow(unused_variable)]

pub use native::arch;

mod native;

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
