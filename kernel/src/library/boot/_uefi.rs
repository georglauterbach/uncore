// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

/// ### UEFI Boot Services Memory Map Size Estimation
///
/// This constant estimates the size (in bytes) of the memory map that
/// is obtained after the UEFI boot services were exited. The kernel
/// will panic if the provided map is larger than this constant.
const UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE: usize = 7000;

/// ### UEFI Boot Services Memory Map
///
/// This array holds the memory map obtained after the UEFI boot
/// services were exited.
#[allow(dead_code)]
pub static UEFI_BOOT_SERVICES_MEMORY_MAP: &[u8; UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE] =
	&[0; UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE];

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
pub fn exit_boot_services()
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
	log_trace!("UEFI system table acquired");

	let memory_map_size = uefi_system_table.boot_services().memory_map_size();
	log_trace!(
		"UEFI boot services memory map size = {} Byte",
		memory_map_size
	);
	assert!(
		memory_map_size < UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE,
		"The UEFI memory map size is smaller than what is statically allocated ({} Byte)",
		UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE
	);

	// TODO exit UEFI boot services
	// let x = match uefi_system_table.exit_boot_services(image, &mut arr)
	// { 	Ok(system_table) => system_table,
	// 	Err(error) => panic!(
	// 		"Could not exit UEFI boot services: {:#?}",
	// 		error
	// 	),
	// };

	// TODO maybe parse memory map into a proper
	// TODO  structure (from the `uefi` crate)?

	log_debug!("[TODO] Exited UEFI boot services");
	log_info!("Boot phase finished");
}
