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

/// ### Architecture Initialization Routine
///
/// This function takes care of the correct initialization of the x86 64Bit architecture.
pub fn initialize()
{
	crate::prelude::log_trace!("Initializing x86_64");
	cpu::initialize();
}
