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

/// TODO
/// 
pub fn initialize()
{
	use crate::prelude::*;
	log_info!("Starting architecture specific initialization");

	cpu::initialize();

	log_info!("Finished Architecture specific initialization");
}
