/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Copyright 2022 The unCORE Kernel Organization */

/*
 * Multiboot 2 assembly headers
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
 * Additional explanations:
 *   1. `.byte` places 8 Bit  (= 1 Byte) (= u8)  of data,
 *      `.word` places 16 Bit (= 2 Byte) (= u16) of data,
 *      `.long` places 32 Bit (= 4 Byte) (= u32) of data,
 *      `.quad` places 64 Bit (= 8 Byte) (= u64) of data
 *
 *  2. In order to boot into "EFI amd64 machine state with boot services
 *     enabled" (3.5 in spec, 2022-01), we must specify additional tags.
 *     
 *     2.1 "EFI boot services tag" leaves UEFI boot services enabled.
 *         Its our task to exit them.
 *
 *     2.2 "EFI amd64 entry address tag of Multiboot2 header tag"
 *
 */

.code64
.extern start

# CONSTANTS
# ------------------------------------------------------------------

.set MAGIC_NUMBER  , 0xE85250D6
.set ARCHITECTURE  , 0x0 # == protected mode i386
.set HEADER_LENGTH , multiboot_2_header_start - multiboot_2_header_end
.set CHECKSUM      , -(MAGIC_NUMBER + ARCHITECTURE + HEADER_LENGTH)

# HEADER DEFINITION
# ------------------------------------------------------------------

.align 8
.section .multiboot2_header

multiboot_2_header_start:
	# required multiboot data
	.long MAGIC_NUMBER
	.long ARCHITECTURE
	.long HEADER_LENGTH
	.long CHECKSUM

	# optional multiboot tags (3.1.12 EFI boot services)
	# see 2.1
	.align 8
	.EFI_boot_services_tags_start:
	.word 0x7
	.word 0x0
	.long .EFI_boot_services_tags_end - .EFI_boot_services_tags_start
	.EFI_boot_services_tags_end:

	# optional multiboot tags (3.1.8 EFI amd64 entry address)
	# see 2.2
	.EFI_amd64_entry_address_tags_start:
	.align 8
	.word 0x9
	.word 0x0
	.long .EFI_amd64_entry_address_tags_end - .EFI_amd64_entry_address_tags_start
	.long start
	.EFI_amd64_entry_address_tags_end:

	# required end tags
	.align 8
	.multiboot_2_end_tags_start:
	.word 0x0
	.word 0x0
	.long .multiboot_2_end_tags_end - .multiboot_2_end_tags_start
	.multiboot_2_end_tags_end:
multiboot_2_header_end:
