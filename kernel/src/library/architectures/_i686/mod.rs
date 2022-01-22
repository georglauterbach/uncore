// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub mod cpu;

/// ### Architecture Initialization Routine
///
/// This function takes care of the correct initialization of the x86 32Bit architecture.
pub fn initialize()
{
	crate::prelude::log_trace!("Initializing i686");
	cpu::initialize();
}
