// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// TODO

use crate::prelude::*;

use x86_64::structures::paging;

/// TODO
pub struct PageTable<'a>(Option<paging::OffsetPageTable<'a>>);

#[cfg(target_arch = "x86_64")]
impl<'a> PageTable<'a>
{
	/// TODO
	#[must_use]
	pub const fn new(page_table: Option<paging::OffsetPageTable<'a>>) -> Self { Self(page_table) }
}

impl<'a> memory::PageAllocation for PageTable<'a>
{
	fn allocate_page<FA>(&mut self, _frame_allocator: FA)
	where
		FA: memory::FrameAllocation,
	{
		unimplemented!()
	}
}

/// TODO
pub struct FrameAllocator(Option<frame_allocation::BootInfoFrameAllocator>);

impl FrameAllocator
{
	/// TODO
	#[must_use]
	pub const fn new(allocator: Option<frame_allocation::BootInfoFrameAllocator>) -> Self
	{
		Self(allocator)
	}
}

impl memory::FrameAllocation for FrameAllocator
{
	fn allocate_frame(&mut self) -> Result<(), ()> { todo!() }
}

/// ### Architecture Specific Virtual Memory Initialization
///
/// This function initializes the virtual memory for the `x86_64` architecture.
pub fn initialize(
	boot_information: &'static bootloader::BootInfo,
) -> (paging::OffsetPageTable, frame_allocation::BootInfoFrameAllocator)
{
	log_info!("Initializing virtual memory for x86_64");

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
		unsafe { frame_allocation::BootInfoFrameAllocator::init(&boot_information.memory_regions) };

	initialize_kernel_heap(&mut frame_allocator, &mut offset_page_table);

	(offset_page_table, frame_allocator)
}

// TODO this is actually for the kernel heap... re-locate it after a proper refactoring
/// TODO
///
/// #### Panics
///
/// TODO
fn initialize_kernel_heap(
	frame_allocator: &mut frame_allocation::BootInfoFrameAllocator,
	offset_page_table: &mut paging::OffsetPageTable,
)
{
	use x86_64::structures::paging::{
		FrameAllocator,
		Mapper,
	};

	log_info!("Initializing (fallback) kernel heap memory");
	let page_range = {
		let heap_start = x86_64::VirtAddr::new(memory::KERNEL_HEAP_START as u64);
		let heap_end = heap_start + memory::KERNEL_HEAP_SIZE - 1u64;
		let heap_start_page = paging::Page::containing_address(heap_start);
		let heap_end_page = paging::Page::containing_address(heap_end);
		paging::Page::range_inclusive(heap_start_page, heap_end_page)
	};

	for page in page_range {
		let frame = frame_allocator.allocate_frame().unwrap();
		let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;
		unsafe {
			offset_page_table
				.map_to(page, frame, flags, frame_allocator)
				.unwrap()
				.flush();
		}
	}
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

/// TODO
mod frame_allocation
{
	use bootloader::boot_info::{
		MemoryRegionKind,
		MemoryRegions,
	};
	use x86_64::structures::paging;

	/// TODO
	pub struct BootInfoFrameAllocator
	{
		/// TODO
		memory_map: &'static MemoryRegions,
		/// TODO
		next:       usize,
	}

	impl BootInfoFrameAllocator
	{
		/// Create a `paging::FrameAllocator` from the passed memory map.
		///
		/// This function is unsafe because the caller must guarantee that the
		/// passed memory map is valid. The main requirement is that all frames
		/// that are marked as `USABLE` in it are really unused.
		pub const unsafe fn init(memory_map: &'static MemoryRegions) -> Self
		{
			Self { memory_map, next: 0 }
		}

		/// Returns an iterator over the usable frames specified in the memory
		/// map.
		fn usable_frames(&self) -> impl Iterator<Item = paging::PhysFrame>
		{
			// get usable regions from memory map
			let regions = self.memory_map.iter();
			let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);
			// map each region to its address range
			let addr_ranges = usable_regions.map(|r| r.start..r.end);
			// transform to an iterator of frame start addresses
			let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
			// create `paging::PhysFrame` types from the start addresses
			frame_addresses.map(|addr| {
				paging::PhysFrame::containing_address(x86_64::PhysAddr::new(addr))
			})
		}
	}

	unsafe impl paging::FrameAllocator<paging::Size4KiB> for BootInfoFrameAllocator
	{
		fn allocate_frame(&mut self) -> Option<paging::PhysFrame>
		{
			let frame = self.usable_frames().nth(self.next);
			self.next += 1;
			frame
		}
	}
}
