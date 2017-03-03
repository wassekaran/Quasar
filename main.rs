#![crate_name = "quasar"]

#![feature(asm)]
#![feature(core_slice_ext)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]
#![deny(warnings)]

pub mod arch;
pub mod kernel;
