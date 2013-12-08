use runtime::cast;
use runtime::Slice;

type Handle = *();

struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32
}

struct TextInputProtocol;

struct SystemTable {
    hdr: TableHeader,
    firmware_vendor: *u16,
    firmware_rev: u32,
    console_in_h: Handle,
    con_in: *TextInputProtocol,
    console_out_h: Handle,
    con_out: *TextOutputProtocol
}

struct TextOutputProtocol {
    reset: *(),
    output_string: extern fn(*TextOutputProtocol, *u16)
}

unsafe fn as_buf(t: &[u16]) -> Slice<u16> {
    cast::transmute(t)
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(image_h: Handle, sys_tb: *SystemTable) -> int {
    unsafe {
        let hello = ['Q' as u16,
                    'U' as u16,
                    'A' as u16,
                    'S' as u16,
                    'A' as u16,
                    'R' as u16];

        let out = (*(*sys_tb).con_out).output_string;
        let sl: Slice<u16> = as_buf(hello);
        out((*sys_tb).con_out, sl.data);

        loop {}
    }
}
