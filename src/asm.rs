use core::arch::global_asm;

global_asm!(
    "
    get_current_directory:
        push rbp
        mov rbp, rsp
        sub rsp, 256
        mov rdi, rsp
        mov rsi, 256
        mov eax, 79
        syscall
        cmp rax, -1
        je error
        mov rdi, 1
        mov rsi, rsp
        mov rdx, rax
        mov eax, 1
        syscall
        add rsp, 256
        mov rsp, rbp
        pop rbp
        ret

    error:
        add rsp, 256
        mov rsp, rbp
        pop rbp
        ret

    "
);
extern "C" {
    fn get_current_directory();
}
// global_asm!(r#"")

// dir_path db './', 0
