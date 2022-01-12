// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;
use super::{
	fake_lock,
	MULTIBOOT2_INFORMATION,
};
use uefi::{
	data_types,
	table,
};

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
pub static mut UEFI_BOOT_SERVICES_MEMORY_MAP: &mut [u8] =
	&mut [0; UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE];

/// ### UEFI Runtime System Table
///
/// Represents the view of the system **after** exiting the UEFI boot
/// services.
static UEFI_SYSTEM_TABLE_RUNTIME: fake_lock::Lock<Option<table::SystemTable<table::Runtime>>> =
	fake_lock::Lock::new(None);

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
/// 3. the statically allocated memory map is too small
/// 4. the UEFI boot services could not be exited cleanly
#[must_use]
pub fn exit_boot_services(
) -> impl ExactSizeIterator<Item = &'static table::boot::MemoryDescriptor> + Clone
{
	use ::core::ffi::c_void;

	let multiboot2_information = MULTIBOOT2_INFORMATION
		.get()
		.as_ref()
		.expect("Could not acquire the multiboot2 information structure");

	let uefi_system_table: table::SystemTable<table::Boot> =
		multiboot2_information.efi_sdt_64_tag().map_or_else(
			|| {
				panic!("Could not acquire the UEFI system table from the \
				        multiboot2 information");
			},
			|uefi_system_table| {
				unsafe {
					table::SystemTable::from_ptr(
						uefi_system_table.sdt_address() as *mut c_void,
					)
				}
				.expect("Acquiring the UEFI system table from pointer did not \
				          succeed")
			},
		);
	log_trace!("Acquired UEFI system table for boot view (temporarily)");

	let memory_map_size = uefi_system_table.boot_services().memory_map_size().map_size;
	log_trace!(
		"UEFI boot services memory map size = {} Byte",
		memory_map_size
	);
	assert!(
		memory_map_size < UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE,
		"The UEFI memory map size is smaller than what is statically allocated ({} Byte)",
		UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE
	);

	let uefi_image_handle_address = multiboot2_information
		.efi_64_ih()
		.expect("No UEFI 64bit image handle provided by the multiboot2 information")
		.image_handle();

	let uefi_image_handle =
		unsafe { data_types::Handle::from_ptr(uefi_image_handle_address as *mut c_void) }
			.expect(
				"Acquiring the UEFI image handle from the handle address did not \
				 succeed",
			);

	let (uefi_system_table_runtime, uefi_memory_map_iterator) = match uefi_system_table
		.exit_boot_services(uefi_image_handle, unsafe { UEFI_BOOT_SERVICES_MEMORY_MAP })
	{
		Ok(completion) => {
			if completion.status().is_success() {
				let (_, result) = completion.split();
				result
			} else {
				panic!(
					"Exiting UEFI boot services resulted in non-successful \
					 completion status: {:#?}",
					completion.status()
				)
			}
		},
		Err(error) => panic!("Could not exit UEFI boot services: {:#?}", error),
	};
	log_debug!("Exited UEFI boot services");

	UEFI_SYSTEM_TABLE_RUNTIME
		.get_mut()
		.replace(uefi_system_table_runtime);
	log_trace!("Acquired UEFI system table for runtime view");

	log_info!("Boot phase finished");
	uefi_memory_map_iterator
}
