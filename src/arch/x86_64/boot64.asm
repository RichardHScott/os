global long_mode_start

section .text
bits 64

long_mode_start:
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    mov ecx, 0x30
.looptest
    inc
    mov eax 0x2f002fff
    or eax, ecx

    mov dword [0xb8000 + ecx * 4], eax
    cmp ecx, 5a
    jmp .looptest 

    hlt