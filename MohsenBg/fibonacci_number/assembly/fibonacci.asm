%use masm

section .text
    global fibonacci ; int fibonacci(int n)

%define n rbp - 4
%define last_value rbp - 8
%define second_last_value rbp - 12

fibonacci:
    push rbp
    mov  rbp, rsp
    sub  rsp, 16              ; reserve space for locals

    mov  dword ptr [n], edi       ; store input n
    mov  dword ptr [second_last_value], 0
    mov  dword ptr [last_value], 1

    ; if n <= 0 return 0
    cmp  dword ptr [n], 0
    jg   check_one
    mov  eax, 0
    jmp  end_fib

check_one:
    ; if n == 1 return 1
    cmp  dword ptr [n], 1
    jne  loop_start
    mov  eax, 1
    jmp  end_fib

loop_start:
    mov  ecx, 2               ; loop counter

loop:
    cmp  ecx, dword ptr [n]
    jg   return_result        ; exit loop if ecx > n

    mov  eax, dword ptr [second_last_value]
    mov  ebx, dword ptr [last_value]
    add  eax, ebx
    mov  dword ptr [second_last_value], ebx
    mov  dword ptr [last_value], eax

    inc  ecx
    jmp  loop

return_result:
    mov  eax, dword ptr [last_value]

end_fib:
    leave
    ret
