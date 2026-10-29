; Unformatted

    global _start

    section .text
_start:
    mov rax, 1                         ; write syscall
    mov rdi, 1                         ; file handle (1 is stdout)
    mov rsi, message                   ; address of string to output
    mov rdx, 13                        ; number of bytes
    syscall

    mov rax, 60                        ; syscall number for exit
    xor rdi, rdi                       ; exit code 0
    syscall

    section .data
message:
    db "Hello, World", 10              ; 10 = newline (0xA)
