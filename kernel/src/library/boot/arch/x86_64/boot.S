.code64
.global start
.extern kernel_main

start:
	xor %rdi, %rdi
	mov %esi, %esi
	mov %eax, %edi
	mov %ebx, %esi

	xor %rsp, %rsp
	xor %rbp, %rbp

	mov $initial_stack_top, %rsp
	mov $initial_stack_top, %rbp

	jmp kernel_main

	stop:
		cli
		hlt

.section .bss, "aw"
.align 16
initial_stack_bottom:
	.zero 0x2000
initial_stack_top:
