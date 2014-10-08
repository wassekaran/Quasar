use runtime::lang::repr::Repr;

mod console {
    use arch::console;

    pub unsafe fn puts(s: *const u8, len: uint) {
        let mut i = 0;
        while i < len {
            console::putcar(*(s + i));
            i += 1;
        }
    }
}

trait Writer {
    fn write(&mut self, buf: &[u8]);
}

pub fn println(s: &str) {
    let slice = s.repr();
    unsafe { console::puts(slice.data, slice.len) }
}
