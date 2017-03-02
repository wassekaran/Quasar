// IO primitives for x86_64
pub use self::ports::out;

mod ports {
    pub trait Ports {
        unsafe fn out(port: u16, value: Self);
    }

    impl Ports for u8 {
        #[inline(always)]
        unsafe fn out(port: u16, value: u8) {
            asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile" );
        }
    }

    impl Ports for u16 {
        #[inline(always)]
        unsafe fn out(port: u16, value: u16) {
            asm!("outw %ax, %dx" :: "{dx}" (port), "{ax}" (value) :: "volatile" );
        }
    }

    pub fn out<T: Ports>(port: u16, value: T) {
        unsafe {
            Ports::out(port, value)
        }
    }
}

pub mod console {
    use super::out;
    use self::Color::*;

    #[allow(dead_code)]
    enum Color {
        Black       = 0x0,
        Blue        = 0x1,
        Green       = 0x2,
        Cyan        = 0x3,
        Red         = 0x4,
        Pink        = 0x5,
        Brown       = 0x6,
        LightGray   = 0x7,
        DarkGray    = 0x8,
        LightBlue   = 0x9,
        LightGreen  = 0xA,
        LightCyan   = 0xB,
        LightRed    = 0xC,
        LightPink   = 0xD,
        Yellow      = 0xE,
        White       = 0xF
    }

    static VIDEO_MEM: u64 = 0xB8000;

    static mut cursor_x: u64 = 0;
    static mut cursor_y: u64 = 0;

    #[inline(always)]
    unsafe fn update_cursor() {
        let offset: u64 = cursor_y * 80 + cursor_x;
        let off_low = offset & 0xFF;
        let off_high = (offset >> 8) & 0xFF;

        out(0x3D4, 0x0Fu8);
        out(0x3D5, off_low as u8);
        out(0x3D4, 0x0Eu8);
        out(0x3D5, off_high as u8);
    }

    #[inline(always)]
    unsafe fn newline() {
        cursor_x = 0;
        cursor_y += 1;
        update_cursor();
    }

    #[inline(always)]
    unsafe fn scroll() {
        let video_ptr = VIDEO_MEM as *mut u8;
        for i in 1 .. 25 {
            for j in 0 .. 80 {
                let orig = i * 80 + j;
                let dest = (i - 1) * 80 + j;
                *video_ptr.offset(dest * 2) = *video_ptr.offset(orig * 2);
                *video_ptr.offset(dest * 2 + 1) = *video_ptr.offset(orig * 2 + 1);
            }
        }
        for i in 0 .. 80 {
            let offset = 24 * 80 + i;
            *video_ptr.offset(offset * 2) = 0x20;
        }
        cursor_y -= 1;
    }

    #[inline(always)]
    unsafe fn forward_cursor() {
        cursor_x += 1;

        if cursor_x >= 80 {
            newline();
        }

        if cursor_y >= 25 {
            scroll();
        }

        update_cursor();
    }

    #[inline(always)]
    unsafe fn do_putcar(c: u8, color: Color) {
        // get video_ptr
        let offset = cursor_y * 80 + cursor_x;
        let video_ptr = (VIDEO_MEM + offset * 2) as *mut u8;
        *video_ptr.offset(1) = color as u8;
        *video_ptr = c;
        forward_cursor();
    }

    #[inline(always)]
    pub fn putcar(c: u8) {
        match c {
            10 => unsafe { newline() },
            _ => unsafe { do_putcar(c, LightGray); }
        }
    }

    pub unsafe fn puts(s: *const u8, len: usize) {
        let mut i = 0;
        while i < len {
            putcar(*s.offset(i as isize));
            i += 1;
        }
    }
}

use core::fmt;

pub struct Console;
impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use core::slice::SliceExt;
        let b = s.as_bytes();
        unsafe { console::puts(b.as_ptr(), b.len()) };
        Ok(())
    }
}
