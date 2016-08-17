global long_mode_start
extern rust_main

section .text
bits 64

long_mode_start:
    call ok

    ;call setup_interrupt_handlers

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    call rust_main

    hlt

setup_interrupt_handlers:
    ;mov qword [interrupt_descriptor_table.base], interrupt_vectors
    ;mov word [interrupt_descriptor_table.limit], 16*100
    
    mov rax, i_ok

    mov ecx, 99
.setup_idt_loop:
    mov ebx, ecx
    shl ebx, 4
    ;add ebx, interrupt_vectors
    mov word [ebx], ax
    shr rax, 8 
    add ebx, 2
    mov word [ebx], 1
    add ebx, 2
    mov byte [ebx], 0
    add ebx, 1
    mov byte [ebx],0b1110
    add ebx, 1 
    mov word [ebx], ax
    add ebx, 2
    shr rax, 16
    mov dword [ebx], eax
    add ebx, 4
    mov dword [ebx], 0

    mov word [0xb8000], cx
    dec ecx
    jnz .setup_idt_loop

    ;lidt [interrupt_descriptor_table]
    ret

ok:
    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    ret

i_ok:
    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
    iret
error:
    ;print 'ERR: ' to screen
    ;followed by error code in al (which is in ascii)
    mov dword [0xb8000], 0x2f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte [0xb800c], al
    hlt

section .bss
align 16

interrupt_descriptor_table:
.limit:
    resb 2
.base:
    resb 4

;interrupt_vectors:
;    resb 16*100
;interrupt_vectors_end: