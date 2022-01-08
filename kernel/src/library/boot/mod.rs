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
pub use _multiboot2::check_and_parse as check_and_parse_multiboot2;

/// ## Handle UEFI
///
/// Handles UEFI related matters (boot services). After entry into
/// `crate::kernel_main(...)`, UEFI boot services are still active and
/// we need to handle and exit them.
mod _uefi;

pub use _uefi::UEFI_BOOT_SERVICES_MEMORY_MAP;
pub use _uefi::exit_boot_services as exit_uefi_boot_services;

// ? ASSEMBLY BOOT CODE
// ? ---------------------------------------------------------------------

// Include order possibly matters for the linker. Make sure to
// pick the correct sequence.

use core::arch::global_asm;

// * x86_64
// * ---------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
global_asm!(
	include_str!("../hardware/_x86_64/boot/start.S"),
	options(att_syntax)
);

#[cfg(target_arch = "x86_64")]
global_asm!(
	include_str!("../hardware/_x86_64/boot/multiboot2.S"),
	options(att_syntax)
);
