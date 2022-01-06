// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// * MODULES
// * ---------------------------------------------------------------------

pub use super::helper::test;

// * STRUCTURES
// * ---------------------------------------------------------------------

pub use super::helper::miscellaneous::KernelInformation;

// * MACROS
// * ---------------------------------------------------------------------

pub use crate::log_trace;
pub use crate::log_debug;
pub use crate::log_info;
pub use crate::log_warning;
pub use crate::log_error;
pub use crate::log_fatal;
pub use crate::log_test;

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
				core::arch::asm!(
					"hlt",
					"cli",
					options(nomem, nostack, preserves_flags)
				);
			}
		}
	}
}
