#[link(name = "kernel", vers = "0.1")];
#[allow(ctypes)];

#[no_std];
#[no_core];

mod util;
mod arch;
mod runtime;

#[no_mangle]
pub unsafe fn main ()
{
    use util::kprintln;

    kprintln("QUASAR");
}
