// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;
use super::fake_lock;
use multiboot2::BootInformation;

/// ### QEMU's Multiboot2 Magic Constant
///
/// This value _should_ be provided by QEMU in the `%eax` register. We
/// forward the value to the kernel's main function.
const MULTIBOOT2_BOOTLOADER_MAGIC_VALUE: u32 = 0x36D7_6289;

/// ### The Multiboot2 Information Structure
///
/// Holds the multiboot2 information provided by the bootloader.
pub static MULTIBOOT2_INFORMATION: fake_lock::Lock<Option<BootInformation>> =
	fake_lock::Lock::new(None);

/// ### Check and Parse Multiboot2 Information
///
/// This function handles the parsing of the multiboot2 structure
/// given to us with a pointer by the assembly boot code.
///
/// #### Panics
///
/// This function panics if
///
/// 1. the magic value of the bootloader
/// [`MULTIBOOT2_BOOTLOADER_MAGIC_VALUE`] 2. the pointer to the
/// multiboot2 information structure is invalid
pub fn check_and_handle(
	multiboot2_bootloader_magic_value: u32,
	multiboot2_boot_information_pointer: u32,
)
{
	log_trace!("Checking multiboot2 bootloader value");
	assert!(
		multiboot2_bootloader_magic_value == MULTIBOOT2_BOOTLOADER_MAGIC_VALUE,
		"The multiboot2 magic value of QEMU does not match"
	);

	log_debug!("Acquiring multiboot2 information structure");
	MULTIBOOT2_INFORMATION.get_mut().replace(
		unsafe { multiboot2::load(multiboot2_boot_information_pointer as usize) }
			.expect("Could not load the multiboot2 information structure"),
	);
}
