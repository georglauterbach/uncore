// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub use __uefi::UEFISystemTableBootTime;
pub use __uefi::UEFIMemoryMap;

pub use __uefi::UEFI_BOOT_SERVICES_MEMORY_MAP;
pub use __uefi::exit_boot_services;

/// ## Handle UEFI
///
/// Handles UEFI related matters (boot services). After entry into
/// `crate::efi_main(...)`, UEFI boot services are still active and
/// we need to handle and exit them.
mod __uefi
{
	use crate::prelude::*;
	use uefi::table;

	/// ### UEFI System Table - Boot
	///
	/// This is the UEFI system table before exiting the boot
	/// services.
	pub type UEFISystemTableBootTime = table::SystemTable<table::Boot>;

	/// ### UEFI System Table - Runtime
	///
	/// This is the UEFI system table after exiting the boot
	/// services.
	type UEFISystemTableRunTime = table::SystemTable<table::Runtime>;

	/// ### UEFI Memory Map Iterator
	///
	/// After exiting the UEFI boot services, this type is
	/// returned from the [`exit_boot_services`] function to
	/// obtain a memory map later.
	pub type UEFIMemoryMap =
		impl ExactSizeIterator<Item = &'static uefi::table::boot::MemoryDescriptor> + Clone;

	/// ### UEFI Boot Services Memory Map Size Estimation
	///
	/// This constant estimates the size (in bytes) of the memory
	/// map that is obtained after the UEFI boot services were
	/// exited. The kernel will panic if the provided map is
	/// larger than this constant.
	const UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE: usize = 7000;

	/// ### UEFI Boot Services Memory Map
	///
	/// This array holds the memory map obtained after the UEFI
	/// boot services were exited.
	pub static mut UEFI_BOOT_SERVICES_MEMORY_MAP: &mut [u8] =
		&mut [0; UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE];

	/// ## Exiting Boot Services
	///
	/// This function will exit the UEFI boot services.
	///
	/// #### Panics
	///
	/// This function panics if
	///
	/// 1. the previously parsed multiboot2 information structure
	/// (see    [`super::_multiboot2::handle_multiboot2`]) could
	/// not be    acquired
	/// 2. the UEFI system table could not be acquired
	/// 3. the statically allocated memory map is too small
	/// 4. the UEFI boot services could not be exited cleanly
	#[must_use]
	pub fn exit_boot_services(
		uefi_image_handle: uefi::Handle,
		uefi_system_table_boot: UEFISystemTableBootTime,
	) -> (UEFISystemTableRunTime, UEFIMemoryMap)
	{
		let memory_map_size = uefi_system_table_boot
			.boot_services()
			.memory_map_size()
			.map_size;
		log_trace!(
			"UEFI boot services memory map size = {} Byte",
			memory_map_size
		);
		assert!(
			memory_map_size < UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE,
			"The UEFI memory map size is smaller than what is statically allocated \
			 ({} Byte)",
			UEFI_BOOT_SERVICES_MEMORY_MAP_SIZE
		);

		let (uefi_system_table_runtime, uefi_memory_map_iterator) =
			match uefi_system_table_boot.exit_boot_services(uefi_image_handle, unsafe {
				UEFI_BOOT_SERVICES_MEMORY_MAP
			}) {
				Ok(completion) => {
					if completion.status().is_success() {
						let (_, result) = completion.split();
						result
					} else {
						panic!(
							"Exiting UEFI boot services resulted in \
							 non-successful completion status: {:#?}",
							completion.status()
						)
					}
				},
				Err(error) => {
					panic!("Could not exit UEFI boot services: {:#?}", error)
				},
			};

		log_debug!("Exited UEFI boot services acquired UEFI system table for runtime view");
		log_info!("Boot phase finished");
		(uefi_system_table_runtime, uefi_memory_map_iterator)
	}
}
