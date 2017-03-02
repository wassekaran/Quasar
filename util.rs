pub unsafe fn kprint(s: &str)
{
    let mut i: int = 0;
    let repr: ::runtime::Slice<u8> = ::runtime::cast::transmute(s);

    while i < repr.len as int
    {
        let p: *char = ::runtime::intrinsics::offset(repr.data, i) as *char;
        ::arch::io::putcar(*p);
        i += 1;
    } 
}

pub fn kprintln(s: &str)
{
    unsafe
    {
        kprint(s);
        ::arch::io::putcar('\n');
    }
}
