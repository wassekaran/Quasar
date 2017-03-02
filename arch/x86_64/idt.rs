// IRQ Handlers defined in assembler code
// This is important because an handler must return in a specific way that
// can't be achieved in Rust code (see handlers.s)
use super::io::{self, out};

extern {
    static _asm_irq_handler_array: [u64 ; IDT_SIZE as usize];
}

pub fn get_irq_handler(num: u16) -> u64 {
    _asm_irq_handler_array[num as usize]
}


#[repr(packed)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
// Struct representing an entry of the IDT table. Basically contains the
// address of an handler and some flags
struct InterruptDescr {
    clbk_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    clbk_mid: u16,
    clbk_high: u32,
    zero2: u32
}

#[repr(packed)]
// Struct representing the IDT register, passed as operand to the lidt
// instruction. Consists of the size of the table in bytes, and the address
// of its start
struct IDTable {
    limit: u16,
    base: *const [InterruptDescr ; IDT_SIZE as usize]
}

const IDT_SIZE: u16 = 256;
static mut idt_init: bool = false;

// The table itself, an array of 256 entries.
// All the entries are statically initialized so that all interrupts are by
// default handled by a function that do nothing.
// Specialized handlers will come later
static mut descriptors: [InterruptDescr ; IDT_SIZE as usize] = [InterruptDescr {
    clbk_low:  0,
    clbk_mid:  0,
    clbk_high: 0,
    selector: 0x08,
    flags: 0x8E,
    zero: 0,
    zero2: 0
} ; IDT_SIZE as usize];

static mut idt_table: IDTable = IDTable {
    limit: 0, 
    base: 0 as *const [InterruptDescr ; IDT_SIZE as usize]
};

pub unsafe fn load_descriptor(num: u16, clbk: u64, flags: u8, selector: u16) {
    if num >= IDT_SIZE {
        return;
    }

    descriptors[num as usize].clbk_low  = (clbk & 0xFFFF) as u16;
    descriptors[num as usize].clbk_mid  = ((clbk >> 16) & 0xFFFF) as u16;
    descriptors[num as usize].clbk_high = ((clbk >> 32) & 0xFFFFFFFF) as u32;
    descriptors[num as usize].selector = selector;
    descriptors[num as usize].flags = flags;
}

#[no_mangle]
// called from ASM
pub extern "C" fn irq_default_handler(irqno: u16) {
    use core::fmt::Write;
    let _ = writeln!(io::Console, "IRQ {}", irqno);
}

pub unsafe fn setup() {
    if idt_init {
        // IDT already initialized
        return;
    }

    idt_init = false;

    // FIXME: this souldn't be necessary (see above)
    idt_table.limit = IDT_SIZE * 8;
    idt_table.base = &descriptors as *const [InterruptDescr ; 256];

    // FIXME: this shouldn't be necessary (see above)
    let mut i = 0;
    while i < IDT_SIZE {
        let clbk_addr = get_irq_handler(i);
        load_descriptor(i, clbk_addr, 0x8E, 0x08);
        i += 1
    }

    // PIC initialization
    out(0x20, 0x11u8);
    out(0xA0, 0x11u8);
    out(0x21, 0x20u8);
    out(0xA1, 0x28u8);
    out(0x21, 0x04u8);
    out(0xA1, 0x02u8);
    out(0x21, 0x01u8);
    out(0xA1, 0x01u8);
    out(0x21, 0x00u8);
    out(0xA1, 0x00u8);

    asm!("lidt ($0)" :: "r" (&idt_table));
    asm!("sti");
}

