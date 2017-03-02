#[inline]
unsafe fn outb(port: u16, value: u8)
{
    asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile" );
}

static VIDEO_MEM: u64 = 0xB8000;

static mut cursor_x: u64 = 0;
static mut cursor_y: u64 = 0;

#[inline]
unsafe fn update_cursor()
{
    let offset: u64 = cursor_y * 80 + cursor_x;
    let off_low = offset & 0xFF;
    let off_high = (offset >> 8) & 0xFF;

    outb(0x3D4, 0x0F);
    outb(0x3D5, off_low as u8);
    outb(0x3D4, 0x0E);
    outb(0x3D5, off_high as u8);
}

#[inline]
unsafe fn newline()
{
    cursor_x = 0;
    cursor_y += 1;

    update_cursor();
}

#[inline]
unsafe fn forward_cursor() 
{
    cursor_x += 1;

    if(cursor_x >= 80)
    {
        newline();
    }

//    if(cursor_y >= 25)
//        scroll(); 

    update_cursor();
}


#[inline]
unsafe fn do_putcar(c: char, color: Color)
{
    /* get video_ptr */

    let offset: u64 = cursor_y * 80 + cursor_x;
    let video_ptr: u64 = VIDEO_MEM + offset * 2;

    *(video_ptr as *mut char) = c;
    *((video_ptr + 1) as *mut u8) = color as u8;   

    forward_cursor();
}

enum Color
{
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15
}

pub fn putcar(c: char) 
{
    /* body */

    match c
    {
        '\n' => unsafe { newline() },

        _ => unsafe { do_putcar(c, LightGray); }
    }
}

