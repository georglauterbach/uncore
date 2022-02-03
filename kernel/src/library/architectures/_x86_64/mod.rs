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

use crate::{
	library,
	prelude::*,
};

/// ### Kernel Main Entrypoint for `x86_64`
///
/// This is the kernel's entry point directly called by the boot-code
/// (written in assembly). We're still in the UEFI boot services are
/// still enabled: it is our job to disable them now.
pub fn kernel_main(_boot_information: &'static mut bootloader::BootInfo) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	#[cfg(test)]
	crate::__test_runner();

	library::architectures::initialize();
	// library::memory::initialize(uefi_memory_map);

	qemu::exit_with_success();
	never_return()
}

/// ### Architecture Initialization Routine
///
/// This function takes care of the correct initialization of the x86 64Bit architecture.
pub fn initialize()
{
	crate::prelude::log_trace!("Initializing x86_64");
	cpu::initialize();
}
