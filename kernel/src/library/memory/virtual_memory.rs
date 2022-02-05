// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library::{
	architectures,
	prelude::*,
};

/// ### Virtual Memory Initialization
///
/// This function takes care of setting up virtual memory properly. It calls the
/// architecture specific setup routines and then
///
/// TODO abstracts over them with kernel specific types, i.e. architecture agnostic ones.
pub fn initialize(boot_information: &boot::Information)
{
	log_info!("Initializing virtual memory");

	architectures::initialize_memory(boot_information.0);

	log_info!("Finished initializing virtual memory");
}
