#[link(name = "kernel", vers = "0.1")];
#[allow(ctypes)];

#[no_std];
#[no_core];

#[no_mangle]
pub unsafe fn main ()
{
    *((0xB8000) as *mut char) = 'Q';
    *((0xB8001) as *mut u8) = 15;
    *((0xB8002) as *mut char) = 'U';
    *((0xB8003) as *mut u8) = 15;
    *((0xB8004) as *mut char) = 'A';
    *((0xB8005) as *mut u8) = 15;
    *((0xB8006) as *mut char) = 'S';
    *((0xB8007) as *mut u8) = 15;
    *((0xB8008) as *mut char) = 'A';
    *((0xB8009) as *mut u8) = 15;
    *((0xB800A) as *mut char) = 'R';
    *((0xB800B) as *mut u8) = 15;

    loop
    {
    }
}
