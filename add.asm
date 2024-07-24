section .data
msg db 'Aloha',0

section .text
global _test
global _add

_add:
    mov eax, edi  ; Move the first argument to eax
    add eax, esi  ; Add the second argument to eax
    ret

_test:
    mov rax, 0x2000004  ; syscall number for sys_write in macOS
    mov rdi, 1          ; file descriptor (stdout)
    mov rsi, msg        ; pointer to the message
    mov rdx, 5          ; message length
    syscall             ; make the syscall
    ret                 ; return to caller
