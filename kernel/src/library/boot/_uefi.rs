// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

/// ## Exiting Boot Services
///
/// This function will exit the UEFI boot services.
///
/// #### Panics
///
/// This function panics if
///
/// 1. the previously parsed multiboot2 information structure (see
///    [`super::_multiboot2::handle_multiboot2`]) could not be
///    acquired
/// 2. the UEFI system table could not be acquired
pub fn exit_uefi_boot_services()
{
	let uefi_system_table: uefi::prelude::SystemTable<uefi::prelude::Boot> =
		super::MULTIBOOT2_INFORMATION._get().as_ref().map_or_else(
			|| {
				panic!("Could not acquire the multiboot2 information structure");
			},
			|structure| {
				structure.efi_sdt_64_tag().map_or_else(
					|| {
						panic!("Could not acquire the UEFI system table");
					},
					|uefi_system_table| unsafe {
						core::mem::transmute(
							uefi_system_table.sdt_address(),
						)
					},
				)
			},
		);

	log_debug!("UEFI system table acquired");

	let memory_map_size = uefi_system_table.boot_services().memory_map_size();

	log_debug!("Memory map size = {}", memory_map_size);

	// let mut _arr: [u8; 6500] = [0; 6500];
	// let x = match uefi_system_table.exit_boot_services(image,
	// &mut arr) { 	Ok(val) => val,
	// 	Err(error) => panic!("Could not exit UEFI boot services:
	// {:#?}", error), };
}
