#![crate_id = "quasar#0.1"]
#![crate_type = "dylib"]

#![feature(globs)]
#![feature(asm)]
#![no_std]

#![deny(deprecated_owned_vector)]
#![deny(managed_heap_memory)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_qualification)]
#![deny(unnecessary_typecast)]
#![deny(unused_result)]
#![deny(warnings)]

pub use native::arch;

pub mod runtime;

mod native;

#[no_mangle]
pub unsafe fn kmain () {
    runtime::io::println("QUASAR");
    arch::idt::setup();
    loop {}
}
