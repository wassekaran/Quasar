pub fn kprint(s: &str)
{
    use runtime::{cast, intrinsics, Slice};

    unsafe
    {
        let mut i: int = 0;
        let repr: Slice<u8> = cast::transmute(s);

        while i < repr.len as int
        {
            let p: *char = intrinsics::offset(repr.data, i) as *char;
            ::arch::io::putcar(*p);
            i += 1;
        }
    }
}

pub fn kprintln(s: &str)
{
    kprint(s);
    ::arch::io::putcar('\n');
}

/*
 * TODO: better implement an itoa() function that converts an integer
 * to string with an arbitrary base, and then some format function
 */

pub fn kprint_hex(a: u64)
{
    let mut i = 0;
    let mut a = a;

    while i < 16
    {
        let d = (a % 16) as u8;

        ::arch::io::putcar(
            if d < 10 { d + '0' as u8 }
            else { d - 10 + 'A' as u8 }
        as char);

        a = a / 16;
        i += 1;
    }
}
