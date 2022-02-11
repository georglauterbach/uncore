// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Architecture-Specific Physical Memory
///
/// Contains types and functionality to interact with physical memory.
pub mod physical;

/// ## Architecture-Specific Virtual Memory
///
/// Contains types and functionality to interact with virtual memory.
pub mod virtual_;

use crate::prelude::*;

use x86_64::structures::paging;

/// ### Architecture Specific Virtual Memory Initialization
///
/// This function initializes the virtual memory for the `x86_64` architecture.
pub fn initialize(
	boot_information: &'static bootloader::BootInfo,
) -> (
	paging::OffsetPageTable,
	physical::frame_allocation::BootInfoFrameAllocator,
)
{
	let physical_memory_offset = boot_information.physical_memory_offset.into_option().map_or_else(
		|| {
			log_error!(
				"Expected a physical memory offset to be present in the boot information \
				 structure"
			);
			exit_kernel(kernel_types::ExitCode::Failure);
		},
		x86_64::VirtAddr::new,
	);

	let offset_page_table = unsafe {
		let level_4_table = get_active_level_4_table(physical_memory_offset);
		paging::OffsetPageTable::new(level_4_table, physical_memory_offset)
	};

	let frame_allocator = unsafe {
		physical::frame_allocation::BootInfoFrameAllocator::new(&boot_information.memory_regions)
	};

	(offset_page_table, frame_allocator)
}

/// ### Page Table Beginning
///
/// Returns a mutable reference to the active level 4 table.
///
/// #### Safety
///
/// "This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior)."
///
///   -- Stolen From
/// "https://os.phil-opp.com/paging-implementation/#accessing-the-page-tables"
unsafe fn get_active_level_4_table(physical_memory_offset: x86_64::VirtAddr)
	-> &'static mut paging::PageTable
{
	let (level_4_table_frame, _) = x86_64::registers::control::Cr3::read();

	let level_4_table_page = physical_memory_offset + level_4_table_frame.start_address().as_u64();
	let page_table_pointer: *mut paging::PageTable = level_4_table_page.as_mut_ptr();

	&mut *page_table_pointer
}
