// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// * MODULES
// * ---------------------------------------------------------------------

pub use super::helper::miscellaneous::boot;
pub use super::helper::miscellaneous::kernel_types;
pub use super::helper::panic;
pub use super::helper::test;

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

/// ### The Event Horizon
///
/// This function is just a nice abstraction of the call to `loop
/// {...}`, making it absolutely clear what the intend of calling this
/// function is, using its name.
///
/// We use the `hlt` instruction to "halt" the CPU to not burn through
/// CPU time, as a call to `loop {}` would do.
pub fn exit_kernel(exit_code: kernel_types::ExitCode) -> !
{
	use super::helper::miscellaneous::{qemu, kernel_types::ExitCode};

	match exit_code {
		ExitCode::Failure => qemu::exit_with_failure(),
		ExitCode::Success => qemu::exit_with_success(),
	}

	loop {
		#[cfg(target_arch = "x86_64")]
		{
			unsafe {
				core::arch::asm!("hlt", "cli", options(nomem, nostack, preserves_flags));
			}
		}
	}
}
