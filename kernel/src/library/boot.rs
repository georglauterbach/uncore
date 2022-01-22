// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub use __uefi::UEFISystemTableBootTime;
pub use __uefi::UEFISystemTableRunTime;
pub use __uefi::UEFIMemoryMap;

pub use __uefi::UEFI_RUNTIME_SERVICES;
pub use __uefi::exit_boot_services;

/// ## Handle UEFI
///
/// Handles UEFI related matters (boot services). After entry into
/// `crate::efi_main(...)`, UEFI boot services are still active and
/// we need to handle and exit them.
mod __uefi
{
	use crate::prelude::*;
	use uefi::{
		table,
		ResultExt,
	};

	/// ### UEFI System Table - Boot
	///
	/// This is the UEFI system table before exiting the boot
	/// services.
	pub type UEFISystemTableBootTime = table::SystemTable<table::Boot>;

	/// ### UEFI System Table - Runtime
	///
	/// This is the UEFI system table after exiting the boot
	/// services.
	pub type UEFISystemTableRunTime = table::SystemTable<table::Runtime>;

	/// ### UEFI Memory Map Iterator
	///
	/// After exiting the UEFI boot services, this type is
	/// returned from the [`exit_boot_services`] function to
	/// obtain a memory map later.
	pub type UEFIMemoryMap =
		impl ExactSizeIterator<Item = &'static table::boot::MemoryDescriptor> + Clone;

	/// ### UEFI Runtime Services Post-Boot
	///
	/// This variable contains - once initialized - the UEFI runtime services to work
	/// with.
	pub static mut UEFI_RUNTIME_SERVICES: kernel_types::GlobalStaticMut<UEFISystemTableRunTime> =
		kernel_types::GlobalStaticMut::new();

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
		let memory_map_maximum_size =
			uefi_system_table_boot.boot_services().memory_map_size().map_size
				+ 8 * ::core::mem::size_of::<table::boot::MemoryDescriptor>();

		let uefi_memory_map_buffer = uefi_system_table_boot
			.boot_services()
			.allocate_pool(table::boot::MemoryType::LOADER_DATA, memory_map_maximum_size)
			.expect_success("Could not allocate memory pool for UEFI memory map buffer");

		let uefi_memory_map_buffer = unsafe {
			::core::slice::from_raw_parts_mut(uefi_memory_map_buffer, memory_map_maximum_size)
		};

		let (uefi_system_table_runtime, uefi_memory_map_iterator) = uefi_system_table_boot
			.exit_boot_services(uefi_image_handle, uefi_memory_map_buffer)
			.expect_success("could not exit UEFI boot services");

		log_debug!("Exited UEFI boot services acquired UEFI system table for runtime view");
		log_info!("Boot phase finished");

		(uefi_system_table_runtime, uefi_memory_map_iterator)
	}
}

/// ### Setup Variables When Global Allocator Is Setup
///
/// This function is called after the UEFI boot services are exited and once a global
/// allocator is set up. It mostly initializes global variables.
pub fn post_boot_setup(uefi_system_table_runtime: UEFISystemTableRunTime)
{
	use crate::prelude::*;
	log_debug!("Running post-boot initialization");

	log_trace!("UEFI Runtime Services are now initialized");
	unsafe {
		UEFI_RUNTIME_SERVICES = kernel_types::GlobalStaticMut::initialize(uefi_system_table_runtime);
	};
}
