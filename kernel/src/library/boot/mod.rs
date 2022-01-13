// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// ? MODULES
// ? ---------------------------------------------------------------------

/// ## Fake Locking for the Multiboot2
///
/// This provides a fake-lock for the time being. This is not a
/// thread-safe lock, as the marker traits `Send` and `Sync` are
/// implemented on them in a fashion that actually violates the
/// traits (as seen by the clippy exception we have to provide).
mod fake_lock;

/// ## Handle Multiboot2
///
/// This module takes care of parsing the multiboot2 information.
mod _multiboot2;

pub use _multiboot2::MULTIBOOT2_INFORMATION;

/// ## Handle UEFI
///
/// Handles UEFI related matters (boot services). After entry into
/// `crate::kernel_main(...)`, UEFI boot services are still active and
/// we need to handle and exit them.
mod _uefi;

pub use _uefi::UEFI_BOOT_SERVICES_MEMORY_MAP;

// ? FUNCTION
// ? ---------------------------------------------------------------------

/// ### Kernel Boot Procedure
///
/// Unifies the kernel boot procedure. Should be called directly after
/// initializing the log to finished the boot boot.
#[must_use]
pub fn boot(
	multiboot2_bootloader_magic_value: u32,
	multiboot2_boot_information_pointer: u32,
) -> impl ExactSizeIterator<Item = &'static uefi::table::boot::MemoryDescriptor> + Clone
{
	_multiboot2::check_and_parse(
		multiboot2_bootloader_magic_value,
		multiboot2_boot_information_pointer,
	);

	_uefi::exit_boot_services()
}
