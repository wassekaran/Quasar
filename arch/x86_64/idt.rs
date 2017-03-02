/*
 * IRQ Handlers defined in assembler code
 * This is important because an handler must return in a specifiw way that
 * can't be achieved in Rust code (see handlers.s)
 */

extern
{
    /* 
     * FIXME: Rustc is unable to cast fn pointers to u64 at compile time 
     */
    fn _irq_default_handler();
}

/* 
 * Struct representing an entry of the IDT table. Basically contains the 
 * address of an handler and some flags
 */

#[packed]
struct InterruptDescr
{
    clbk_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    clbk_mid: u16,
    clbk_high: u32,
    zero2: u32
}

/*
 * Struct representing the IDT register, passed as operand to the lidt
 * instruction. Consists of the size of the table in bytes, and the address
 * of its start
 */

#[packed]
struct IDTable
{
    limit: u16,
    base: *[InterruptDescr, ..IDT_SIZE]
}

static IDT_SIZE: u16 = 256;
static mut idt_init: bool = false;

/*
 * The table itself, an array of 256 entries.
 * All the entries are statically initialized so that all interrupts are by
 * default handled by a function that do nothing. 
 * Specialized handlers will come later
 */

static mut descriptors: [InterruptDescr, ..IDT_SIZE] = [InterruptDescr 
{
    /* FIXME: the commented expressions here should compile */

    clbk_low:  0, //_irq_default_handler as u16,
    clbk_mid:  0, //((_irq_default_handler as u64 >> 16) & 0xFFFF) as u16,
    clbk_high: 0, //((_irq_default_handler as u64 >> 32) & 0xFFFF) as u32,

    selector: 0x08,
    flags: 0x8E,

    zero: 0,
    zero2: 0
}, ..IDT_SIZE];

/* 
 * FIXME: this should compile but unsafe blocks in constant expressions
 * is currently unimplemented, see #9852 
 */

/*
static mut table: IDTable = unsafe 
    { 
        IDTable 
        { 
            limit: IDT_SIZE * 8,
            base: &descriptors  
        }
     }; 
*/

/* FIXME: the version above should be used */
/* nomangle ensure it will be accessible by plain ASM */

#[no_mangle]
pub static mut idt_table: IDTable = IDTable 
{ 
    limit: 0, 
    base: 0 as *[InterruptDescr, ..IDT_SIZE] 
};

pub unsafe fn load_descriptor(num: u16, clbk: u64, flags: u8, selector: u16)
{
    if(num >= IDT_SIZE)
    {
        return;
    }

    descriptors[num].clbk_low  = (clbk & 0xFFFF) as u16;
    descriptors[num].clbk_mid  = ((clbk >> 16) & 0xFFFF) as u16;
    descriptors[num].clbk_high = ((clbk >> 32) & 0xFFFFFFFF) as u32;
    descriptors[num].selector = selector;
    descriptors[num].flags = flags;
}

/* called from ASM */

#[no_mangle]
pub extern "C" fn irq_default_handler(no: u16) { }

pub unsafe fn setup() 
{
    if idt_init
    {
        /* IDT already initialized */
        return;
    }

    idt_init = false;

    /* FIXME: this souldn't be necessary (see above) */

    idt_table.limit = IDT_SIZE * 8 as u16;
    idt_table.base = &descriptors as *[InterruptDescr, ..256];

    /* FIXME: this shouldn't be necessary (see above) */

    let mut i = 0;

    while i < IDT_SIZE
    { 
        let clbk_addr = _irq_default_handler as u64;
        load_descriptor(i, clbk_addr, 0x8E, 0x08);
        i += 1
    }

    /* PIC initialization */

    ::arch::io::outb(0x20, 0x11);
    ::arch::io::outb(0xA0, 0x11);
    ::arch::io::outb(0x21, 0x20);
    ::arch::io::outb(0xA1, 0x28);
    ::arch::io::outb(0x21, 0x04);
    ::arch::io::outb(0xA1, 0x02);
    ::arch::io::outb(0x21, 0x01);
    ::arch::io::outb(0xA1, 0x01);
    ::arch::io::outb(0x21, 0x0);
    ::arch::io::outb(0xA1, 0x0);

    asm!("lidt (idt_table)");
    asm!("sti");

 //   asm!("divb 0");
}

