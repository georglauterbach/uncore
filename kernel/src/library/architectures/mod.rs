// SPDX-License-Identifier: GPL-3.0-or-later

// ? MODULES
// ? ---------------------------------------------------------------------

// * x86 64 Bit
// * ---------------------------------------------------------------------

/// ## The x86 64 Bit Architecture
///
/// This module contains x86 64 Bit specific initialization and setup
/// code - compiled conditionally.
#[cfg(target_arch = "x86_64")]
mod _x86_64;

#[cfg(target_arch = "x86_64")] pub use _x86_64::kernel_main;
#[cfg(target_arch = "x86_64")] pub use _x86_64::memory;

// ? EXPORTED FUNCTIONS
// ? ---------------------------------------------------------------------

/// ### Architecture-Specific Initialization
///
/// This function uses conditional compilation to initialize the chosen target
/// architecture.
pub fn initialize()
{
	use crate::prelude::*;

	log_info!("Starting architecture specific initialization");

	#[cfg(target_arch = "x86_64")]
	_x86_64::initialize();
	#[cfg(target_arch = "i686")]
	_i686::initialize();
	#[cfg(target_arch = "aarch64")]
	_aarch64::initialize();

	log_info!("Finished Architecture specific initialization");
}
