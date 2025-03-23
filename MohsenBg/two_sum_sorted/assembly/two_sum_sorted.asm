%use masm

section .data

section .text
    global two_sum_sorted ; int* two_sum_sorted(int* numbers, int numbers_size, int target, int* returnSize)
    extern malloc

%define numbers rbp - 8            ; 8    rdi     int *
%define numbers_size rbp - 12      ; 4    rsi     int
%define target rbp - 16            ; 4    rdx     int
%define return_size rbp - 24       ; 8    rcx     int *
%define left rbp - 28              ; 4     -      int
%define right rbp - 32             ; 4     -      int
%define sum rbp - 36               ; 4     -      int
%define result rbp - 44            ; 8     -      int *

two_sum_sorted:
    push rbp
    mov rbp, rsp
    sub rsp, 48  

    ; *numbers

    mov qword ptr [numbers], rdi       

    ; numbers_size
    mov dword ptr [numbers_size], esi  

    ; target = 0
    mov dword ptr [target], edx       

    ; *return_size
    mov qword ptr [return_size], rcx   

    ; left = 0;
    mov dword ptr [left], 0            

    ; right = numbers_size - 1;
    mov eax, dword ptr [numbers_size]   
    dec eax                          
    mov dword ptr [right], eax         

loop:
    ; Check if left pointer >= right pointer (no solution)
    mov eax,dword ptr [left]
    cmp eax, dword ptr [right]
    jge failed
    

    ; numbers[left]
    mov rax, qword ptr [numbers]       
    mov ebx, dword ptr [left]  
    mov edx, [rax + rbx * 4]

    ; numbers[right] 
    mov ecx, dword ptr [right]           
    mov ebx, [rax + rcx * 4] 

    ; sum = numbers[left] + numbers[right]
    add edx, ebx
    mov dword ptr [sum], edx

    ; if (sum == target)
    mov eax, dword ptr [sum]
    cmp eax, dword ptr [target]
    je success

    ; if (sum > target)
    mov dword ptr [sum], eax
    cmp eax, dword ptr [target]
    jg dec_right_pointer

    ; if (sum < target)
    jl inc_left_pointer

dec_right_pointer:
    dec dword ptr [right]
    jmp loop

inc_left_pointer:
    inc dword ptr [left]
    jmp loop

success:
    ; malloc(sizeof(int) * 2) ===>  4 * 2 = 8
    mov rdi, 8 
    call malloc 
    mov qword ptr[result], rax  
    
    ; result[0] = left + 1
    inc dword ptr [left]
    mov ebx,dword ptr [left]
    mov dword ptr [rax],   ebx 
   
    ; result[1] = right + 1
    inc dword ptr [right]
    mov ebx, dword ptr [right]
    mov [rax + 4], ebx


    ; * return_size = 2
    mov rbx, qword ptr [return_size]
    mov dword ptr [rbx], 2
    jmp done

failed:
    mov qword ptr [result], 0
    mov dword ptr [return_size], 0
    jmp done

done:
    mov rax, qword ptr [result]
    leave
    ret




; C code
; int* two_sum_sorted(int* numbers, int numbers_size, int target, int* returnSize) {
;     int left = 0;
;     int right = numbers_size - 1;
    
    ;     static int result[2];  
    
;     while (left < right) {
;         int sum = numbers[left] + numbers[right];
        
;         if (sum == target) {
;             result[0] = left + 1;  // 1-based index
;             result[1] = right + 1; // 1-based index
;             *returnSize = 2;        // size of the result array
;             return result;
;         }
        
;         if (sum > target) {
;             right--;
;         } else {
;             left++;
;         }
;     }
    
;     *returnSize = 0;  
;     return NULL;
; }
