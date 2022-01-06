/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Copyright 2022 The unCORE Kernel Organization */

/*
 * Custom linker script for building with multiboot2 and `cargo`.
 * Makes sure the multiboot2-header is at the beginning of the file.
 */

ENTRY(start)

SECTIONS {

	/*
		FIXME, see
		https://github.com/phip1611/multiboot2-binary-rust/blob/main/src/rust/rust-kernel/link.ld
	*/
	. = 8M;

	. = ALIGN(8);

	.text . :
	{
		/* see https://stackoverflow.com/questions/68475415/ */
		*(.multiboot2_header)
		*(.text .text.*)
	}

	.rodata :
	{
		*(.rodata .rodata.*)
	}

	.data :
	{
		*(.data .data.*)
	}

	.bss :
	{
		*(COMMON)
		*(.bss .bss.*)
	}

}
