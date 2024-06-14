.section .text
.globl open_dir
.globl read_dir
.globl close_dir

open_dir:
    push rbx
    push rbp
    mov rbx, rdi
    mov rax, 1
    pop rbp
    pop rbx
    ret

read_dir:
    push rbx
    push rbp
    mov rbx, rdi
    mov rcx, rsi
    mov rdx, rdx
    mov rax, 64
    pop rbp
    pop rbx
    ret

close_dir:
    push rbx
    push rbp
    mov rbx, rdi
    mov rax, 0
    pop rbp
    pop rbx
    ret
