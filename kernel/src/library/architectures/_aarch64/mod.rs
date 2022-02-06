// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub mod cpu;

use crate::prelude::*;

/// ### Kernel Main Entrypoint for `aarch64`
///
/// This is the kernel's architecture-specific entry point directly called by the
/// bootloader.
#[cfg(not(test))]
pub fn kernel_main() -> ! { crate::kernel_main(&boot::Information::Aarch64) }

/// ### Kernel Main Entrypoint for `aarch64` During Tests
///
/// This is the kernel's architecture-specific entry point directly called by the
/// bootloader during tests.
#[cfg(test)]
pub fn kernel_main() -> ! { crate::kernel_main(&boot::Information::Aarch64) }

/// ### Architecture Initialization Routine
///
/// This function takes care of the correct initialization of the ARM 64Bit architecture.
pub(super) fn initialize()
{
	crate::prelude::log_debug!("Initializing aarch64");
	cpu::initialize();
}
