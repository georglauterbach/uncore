// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## `x86_64` CPU Setup
///
/// Provides general CPU setup and exception as well as interrupt
/// handlers.
mod cpu;

/// ## Virtual Memory Implementation
///
/// This module contains the virtual memory / paging abstractions for
/// `x86_64`.
mod memory;

use crate::library::prelude::*;

/// ### Kernel Main Entrypoint for `x86_64`
///
/// This is the kernel's architecture-specific entry point directly called by the
/// bootloader.
#[cfg(not(test))]
pub fn kernel_main(boot_information: &'static mut bootloader::BootInfo) -> !
{
	crate::kernel_main(&boot_information.into())
}

/// ### Kernel Main Entrypoint for `x86_64` During Tests
///
/// This is the kernel's architecture-specific entry point directly called by the
/// bootloader during tests.
#[cfg(test)]
pub fn kernel_main(boot_information: &'static mut bootloader::BootInfo) -> !
{
	crate::kernel_main(&boot_information.into())
}

/// ### Architecture Initialization Routine
///
/// This function takes care of the correct initialization of the x86 64Bit architecture.
pub(super) fn initialize()
{
	crate::prelude::log_trace!("Initializing x86_64");
	cpu::initialize();
}

impl From<&'static mut bootloader::BootInfo> for boot::Information
{
	fn from(boot_information: &'static mut bootloader::BootInfo) -> Self
	{
		Self::X86_64(boot_information)
	}
}
