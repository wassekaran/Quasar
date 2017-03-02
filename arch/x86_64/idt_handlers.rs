use super::IDT_SIZE;

extern {
    static _asm_irq_handler_array: [u64 ; IDT_SIZE as usize];
}

pub fn get_irq_handler(num: u16) -> u64 {
    _asm_irq_handler_array[num as usize]
}
