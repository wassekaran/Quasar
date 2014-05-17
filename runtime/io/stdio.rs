use runtime::io::console;
use runtime::lang::repr::Repr;

pub fn println(s: &str) {
    let slice = s.repr();
    unsafe { console::puts(slice.data, slice.len) }
}
