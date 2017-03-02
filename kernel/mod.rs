use arch;
use core::fmt::{self, Write};

#[no_mangle]
pub unsafe fn kmain () {
    let _ = writeln!(arch::io::Console, "QUASAR");
    arch::idt::setup();
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    let _ = writeln!(arch::io::Console, "kernel panic");
    loop {}
}
