#[link(name = "kernel", vers = "0.1")];
#[feature(globs)];
#[allow(ctypes)];

#[no_std];
#[no_core];

pub mod util;
pub mod arch;
pub mod runtime;

#[no_mangle]
pub unsafe fn main ()
{
    use util::kprintln;

    kprintln("QUASAR");
    arch::idt::setup();
    loop {}
}
