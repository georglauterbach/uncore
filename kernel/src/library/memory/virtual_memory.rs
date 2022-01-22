// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// TODO remove unneeded `#[allow(...)]` statements and clean up
use crate::library::{
	self,
	prelude::*,
};

// https://github.com/rust-osdev/bootloader/blob/main/src/bin/uefi.rs#L37

/// TODO add documentation comment here
#[allow(clippy::needless_pass_by_value)]
pub(super) fn initialize(_uefi_memory_map: library::boot::UEFIMemoryMap)
{
	log_info!("Initializing virtual memory");
}
