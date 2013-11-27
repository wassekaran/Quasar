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

%macro  ASM_IRQ_HANDLER 1

[GLOBAL _asm_handler_%1]

    _asm_irq_handler_%1:
        push word %1
        call irq_default_handler
        add rsp, 2
        mov al, 0x20
        out 0x20, al
        iretq

%endmacro

%assign i 0
%rep    256
    ASM_IRQ_HANDLER i
%assign i i+1
%endrep

%unmacro ASM_IRQ_HANDLER 1
%macro   ASM_IRQ_HANDLER 1
    dq _asm_irq_handler_%1
%endmacro

[GLOBAL _asm_irq_handler_array]

_asm_irq_handler_array:
%assign i 0
%rep    256
    ASM_IRQ_HANDLER i
%assign i i+1
%endrep
