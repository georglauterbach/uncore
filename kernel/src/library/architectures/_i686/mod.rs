// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub mod cpu;

use crate::prelude::*;

/// ### Kernel Main Entrypoint for `i686`
///
/// This is the kernel's architecture-specific entry point directly called by the
/// bootloader.
#[cfg(not(test))]
pub fn kernel_main() -> ! { crate::kernel_main(&boot::Information::I686) }

/// ### Kernel Main Entrypoint for `i686` During Tests
///
/// This is the kernel's architecture-specific entry point directly called by the
/// bootloader during tests.
#[cfg(test)]
pub fn kernel_main() -> ! { crate::kernel_main(&boot::Information::I686) }

/// ### Architecture Initialization Routine
///
/// This function takes care of the correct initialization of the x86 32Bit architecture.
pub(super) fn initialize()
{
	crate::prelude::log_debug!("Initializing i686");
	cpu::initialize();
}
