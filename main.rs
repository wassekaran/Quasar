#![crate_name = "quasar"]

#![feature(asm)]
#![feature(core_slice_ext)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]

#![allow(non_upper_case_globals)]
#![allow(safe_extern_statics)]
#![deny(warnings)]

pub mod arch;
pub mod kernel;
