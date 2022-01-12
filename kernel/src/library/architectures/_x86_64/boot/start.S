/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Copyright 2022 The unCORE Kernel Organization */

/*
 * Boot-code for multiboot2
 *   written in GNU Assembly (GAS) syntax / dialect
 *   compiled, assembled and linked by Rust
 * 
 * The multiboot 2 specification can be found under
 * https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
 *
 * Additonal sources used:
 *   - https://os.phil-opp.com/multiboot-kernel/
 *   - https://github.com/winksaville/baremetal-x86_64/blob/master/mb2.S
 *   - https://github.com/phip1611/multiboot2-binary-rust
 *
 */

.code64
.global start
.extern kernel_main

.section .text

	start:
		# We call `kernel_main` with two parameters:
		#   1. multiboot2 magic value
		#   2. pointer to multiboot2 information structure
		#
		# Remember the SYSTEM V x86_64 calling convention:
		#   he first argument is stored in %edi, the second
		#   one is stored in %esi, etc.
		xorq %rdi, %rdi
		xorq %rsi, %rsi

		movl %eax, %edi
		movl %ebx, %esi

		# Stack setup happens here (stack grows downwards,
		#   from high to low addresses)
		#
		# GRUB already used the stack provided by the UEFI
		#   firmware and the multiboot2 specification also
		#   says that our application needs to set up its
		#   own stack.
		xorq %rsp, %rsp
		xorq %rbp, %rbp

		movl $initial_stack_top - 8, %esp
		movl $initial_stack_top - 8, %ebp

		jmp kernel_main

		.stop:
			cli
			hlt

.section .bss

	.align 16
	initial_stack_bottom:
		.fill 0x40000
	initial_stack_top:
