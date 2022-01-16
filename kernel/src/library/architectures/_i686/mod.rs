// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub mod cpu;

/// TODO
/// 
pub fn initialize()
{
	use crate::prelude::*;
	log_info!("Starting architecture specific initialization");

	cpu::initialize();

	log_info!("Finished Architecture specific initialization");
}
