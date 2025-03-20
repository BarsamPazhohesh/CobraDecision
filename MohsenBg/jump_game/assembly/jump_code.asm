%use masm
section .text
    global jump_game ; int canJump(int* nums, int numsSize) 

; int max(int num1, int num2);
max:
    push rbp
    mov rbp, rsp
    
    ; if num1 > num2
    cmp edi, esi
    jge num1_bigger
    
    ; else
    mov eax, esi
    jmp end_max_function

num1_bigger:
    mov eax, edi

end_max_function:
    leave
    ret



; int canJump(int* nums, int numsSize) 
%define nums rbp - 16
%define nums_size rbp - 8
%define max_reach rbp - 4

jump_game:
    push rbp
    mov rbp, rsp
    sub rsp, 16  

    ; *nums
    mov qword ptr [nums], rdi       
    
    ;int nums_size
    mov dword ptr [nums_size], esi  
    
    ; max_reach = 0
    mov dword ptr [max_reach], 0    

    xor ecx, ecx   ; i = 0

loop:

    ; if i >= nums_size, return false
    cmp ecx, dword ptr [nums_size]
    jge failed     

    ; if i > max_reach, return false
    cmp ecx, dword ptr [max_reach]
    jg failed      

    ; max_reach = max(max_reach, nums[i] + i)
    mov edi, dword ptr [max_reach]   
    mov rbx, qword ptr [nums]        
    mov esi, dword ptr [rbx + rcx * 4]  
    add esi, ecx                     
    call max
    mov dword ptr [max_reach], eax   

    ; nums_size - 1
    mov ebx, dword ptr [nums_size]
    dec ebx     

    ; if max_reach >= nums_size - 1, return true
    cmp eax, ebx
    jge success  

    ;i++
    inc ecx
    jmp loop

failed:
    ; false
    mov eax, 0
    jmp end_jump_game_function

success:
    ; true
    mov eax, 1

end_jump_game_function:
    leave   
    ret


; C code
; int max(int num1, int num2)
; {
;     return (num1 >= num2) ? num1 : num2;
; }

; bool canJump(int *nums, int numsSize)
; {
;     int max_reach = 0;
;     for (int i = 0; i < numsSize; i++) 
;     {
;         if (i > max_reach) 
;             return false;
        
;         max_reach = max(max_reach, nums[i] + i); 

;         if (max_reach >= numsSize - 1) 
;             return true;
;     }

;     return false;  
; }
