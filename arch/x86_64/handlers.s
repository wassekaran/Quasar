[BITS 64]
[GLOBAL _irq_default_handler]
[EXTERN irq_default_handler]

; IRQ handlers must be implemented in raw ASM to be able
; to use the iretq instruction to return from the handler
; Rust compiler uses "retq" instead, which is not suitable

_irq_default_handler:
    call irq_default_handler
    mov al, 0x20
    out 0x20, al
    iretq
