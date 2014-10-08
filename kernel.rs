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

pub mod runtime;

mod native;

#[no_mangle]
pub unsafe fn kmain () {
    runtime::io::println("QUASAR");
    arch::idt::setup();
    loop {}
}
