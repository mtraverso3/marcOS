[bits 32]
[extern main] ; Define calling point. same name as kernel.c's 'main' function
call main ; Calls the C function. The linker will know where it is placed in memory
jmp $
