// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// TODO
pub mod physical_memory;

/// TODO
pub mod virtual_memory;

use crate::{
	prelude::*,
	library::memory::physical_memory::FrameAllocation,
};

use x86_64::structures::paging;

use physical_memory::frame_allocation;

/// ### Architecture Specific Virtual Memory Initialization
///
/// This function initializes the virtual memory for the `x86_64` architecture.
pub fn initialize(
	boot_information: &'static bootloader::BootInfo,
) -> (paging::OffsetPageTable, frame_allocation::BootInfoFrameAllocator)
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

	let mut offset_page_table = unsafe {
		let level_4_table = get_active_level_4_table(physical_memory_offset);
		paging::OffsetPageTable::new(level_4_table, physical_memory_offset)
	};

	let mut frame_allocator =
		unsafe { frame_allocation::BootInfoFrameAllocator::new(&boot_information.memory_regions) };

	// initialize_kernel_heap(&mut frame_allocator, &mut offset_page_table);

	(offset_page_table, frame_allocator)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn get_active_level_4_table(physical_memory_offset: x86_64::VirtAddr)
	-> &'static mut paging::PageTable
{
	let (level_4_table_frame, _) = x86_64::registers::control::Cr3::read();

	let level_4_table_page = physical_memory_offset + level_4_table_frame.start_address().as_u64();
	let page_table_pointer: *mut paging::PageTable = level_4_table_page.as_mut_ptr();

	&mut *page_table_pointer
}

// TODO this is actually for the kernel heap... re-locate it after a proper refactoring
/// #### Panics
///
/// TODO
pub fn initialize_kernel_heap(
	// frame_allocator: &mut frame_allocation::BootInfoFrameAllocator,
	offset_page_table: &mut paging::OffsetPageTable,
)
{
	use x86_64::structures::paging::{
		Mapper,
	};

	log_debug!("Initializing (fallback) kernel heap memory");
	let page_range = {
		let heap_start = x86_64::VirtAddr::new(memory::heap::KERNEL_HEAP_START as u64);
		let heap_end = heap_start + memory::heap::KERNEL_HEAP_SIZE - 1u64;
		let heap_start_page = paging::Page::containing_address(heap_start);
		let heap_end_page = paging::Page::containing_address(heap_end);
		paging::Page::range_inclusive(heap_start_page, heap_end_page)
	};

	log_warning!("namd");

	use crate::prelude::memory::physical_memory::KERNEL_FRAME_ALLOCATOR;

	let x = unsafe { KERNEL_FRAME_ALLOCATOR.get_mut().unwrap() };
	log_warning!("naaaaamd");

	for page in page_range {
		let frame: crate::prelude::memory::physical_memory::Frame<
			crate::prelude::memory::virtual_memory::ChunkSizeDefault,
		> = x.allocate_frame().unwrap();
		// let frame = frame_allocator.allocate_frame().unwrap();
		let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;
		unsafe {
			offset_page_table
				.map_to(page, frame.into(), flags, &mut KERNEL_FRAME_ALLOCATOR.get_mut().unwrap().0 )
				.unwrap()
				.flush();
		}
	}
}
