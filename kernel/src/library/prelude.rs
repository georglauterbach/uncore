// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// * MODULES
// * ---------------------------------------------------------------------

pub use super::helper::test;
pub use super::helper::miscellaneous::kernel_types;
pub use super::helper::miscellaneous::qemu;
pub use super::helper::miscellaneous::boot;

// * STRUCTURES
// * ---------------------------------------------------------------------

pub use super::helper::miscellaneous::KernelInformation;

// * MACROS
// * ---------------------------------------------------------------------

pub use log::trace as log_trace;
pub use log::debug as log_debug;
pub use log::info as log_info;
pub use log::warn as log_warning;
pub use log::error as log_error;

// * FUNCTIONS
// * ---------------------------------------------------------------------

pub use super::helper::panic::panic_callback;

/// ### The Event Horizon
///
/// This function is just a nice abstraction of the call to `loop
/// {...}`, making it absolutely clear what the intend of calling this
/// function is, using its name.
///
/// We use the `hlt` instruction to "halt" the CPU to not burn through
/// CPU time, as a call to `loop {}` would do.
#[inline]
pub fn never_return() -> !
{
	loop {
		#[cfg(target_arch = "x86_64")]
		{
			unsafe {
				core::arch::asm!("hlt", "cli", options(nomem, nostack, preserves_flags));
			}
		}
	}
}
