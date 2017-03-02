#![crate_name = "quasar"]

#![feature(asm)]
#![feature(core_slice_ext)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]

#![allow(legacy_directory_ownership)]
#![allow(non_upper_case_globals)]
#![allow(safe_extern_statics)]
#![deny(warnings)]

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
