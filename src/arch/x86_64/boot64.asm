global long_mode_start
extern rust_main

section .text
bits 64

long_mode_start:
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    call rust_main

    hlt

ok:
    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    mov dword [0xb8004], 0x2f4f2f50
    mov dword [0xb8008], 0x2f502f4f
    hlt
error:
    ;print 'ERR: ' to screen
    ;followed by error code in al (which is in ascii)
    mov dword [0xb8000], 0x2f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte [0xb800c], al
    hlt