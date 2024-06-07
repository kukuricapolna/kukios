section .bss
    input resb 128

section .data
    prompt db 'Enter your name: ', 0
    newline db 0xa
    hello db 'Hello, World!', 0xA, 0xA, 'Hello, '  ; The string to print followed by a newline character

section .text
    global _start                  ; Entry point for the program

_start:
    ; Write the string to stdout
    mov rax, 0x2000004             ; System call number for sys_write in macOS
    mov rdi, 1                     ; File descriptor 1 is stdout
    mov rsi, prompt                ; Address of the string to output
    mov rdx, 17                     ; Number of bytes to write (length of "Hello, World!\n")
    syscall                        ; Invoke the system call

    mov rax, 0x2000003
    mov rdi, 0
    mov rsi, input
    mov rdx, 128
    syscall

    mov rax, 0x2000004
    mov rdi, 1
    mov rsi, newline
    mov rdx, 128
    syscall

    ; Exit the program
    mov rax, 0x2000001             ; System call number for sys_exit in macOS
    xor rdi, rdi                   ; Return code 0
    syscall                        ; Invoke the system call
