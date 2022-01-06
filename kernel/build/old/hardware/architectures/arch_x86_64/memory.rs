// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// TODO work in progress

use x86_64::{
	structures::paging::{
		// PageTable,
		OffsetPageTable,
		// Translate,
		FrameAllocator,
		Size4KiB,
		PhysFrame,
		Mapper,
		Page,
	},
	PhysAddr,
	VirtAddr,
};

/// ### Initialize Memory for `x86_64`
///
/// This function initialized the memory for the `x86_64` target
/// platform.
pub fn init(boot_information: &'static bootloader::BootInfo)
{
	let mut offset_page_table = unsafe { create_offset_page_table(boot_information) };

	let mut frame_allocator =
		unsafe { CustomFrameAllocator::init(&boot_information.memory_regions) };

	let page = Page::containing_address(VirtAddr::new(0x0DEA_DBEA_F000));
	create_example_mapping(page, &mut offset_page_table, &mut frame_allocator);
	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_F021_F077_F065_F04E) };
}

/// ### Initialize a new `OffsetPageTable`
///
/// This function crates the page table structure for `uncORE`. As the
/// complete physical memory is mapped by the bootloader, we cam just
/// use an offset to calculate the first level 4 page table.
///
/// #### Safety
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called
/// once to avoid aliasing `&mut` references (which is undefined
/// behavior).
unsafe fn create_offset_page_table(
	boot_information: &bootloader::BootInfo,
) -> OffsetPageTable<'static>
{
	let physical_memory_mapping_offset =
		if let Some(address) = boot_information.physical_memory_offset.into_option() {
			VirtAddr::new(address)
		} else {
			crate::log_fatal!("Physical memory offset non-existent");
			panic!("Memory offset should not be non-existent");
		};

	let (level_4_page_table_frame, _) = x86_64::registers::control::Cr3::read();
	let level_4_page_table_frame_physical_address = level_4_page_table_frame.start_address();
	let level_4_page_table_frame_virtual_address =
		physical_memory_mapping_offset + level_4_page_table_frame_physical_address.as_u64();
	let level_4_page_table = level_4_page_table_frame_virtual_address.as_mut_ptr();

	crate::log_trace!(
		"Page table starts at {:?} / {:?}",
		level_4_page_table_frame_physical_address,
		level_4_page_table_frame_virtual_address
	);

	OffsetPageTable::new(&mut *level_4_page_table, physical_memory_mapping_offset)
}

// ! ---------------------------------------------------------------------

// TODO

// https://os.phil-opp.com/paging-implementation/#creating-a-new-mapping

/// Just some doc
pub fn create_example_mapping(
	page: Page,
	mapper: &mut OffsetPageTable,
	frame_allocator: &mut impl FrameAllocator<Size4KiB>,
)
{
	use x86_64::structures::paging::PageTableFlags as Flags;

	let frame = PhysFrame::containing_address(PhysAddr::new(0xB8000));
	let flags = Flags::PRESENT | Flags::WRITABLE;

	let map_to_result = unsafe {
		// FIXME: this is not safe, we do it only for testing
		mapper.map_to(page, frame, flags, frame_allocator)
	};
	map_to_result.expect("map_to failed").flush();
}

use bootloader::boot_info::{
	MemoryRegions,
	MemoryRegionKind,
};
// use bootloader::bootinfo::MemoryMap;

/// A `CustomFrameAllocator` that returns usable frames from the
/// bootloader's memory map.
pub struct CustomFrameAllocator
{
	/// morjen
	memory_map: &'static MemoryRegions,
	/// namd
	next:       usize,
}

impl CustomFrameAllocator
{
	/// Create a `CustomFrameAllocator` from the passed memory
	/// map.
	///
	/// This function is unsafe because the caller must guarantee
	/// that the passed memory map is valid. The main requirement
	/// is that all frames that are marked as `USABLE` in it are
	/// really unused.
	///
	/// #### Safety
	///
	/// TODO
	#[must_use]
	pub const unsafe fn init(memory_map: &'static MemoryRegions) -> Self
	{
		Self {
			memory_map,
			next: 0,
		}
	}

	/// namd
	/// TODO
	fn usable_frames(&self) -> impl Iterator<Item = PhysFrame>
	{
		// get usable regions from memory map
		let regions = self.memory_map.iter();

		let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);

		// map each region to its address range
		let addr_ranges = usable_regions.map(|r| r.start..r.end);

		// transform to an iterator of frame start addresses
		let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
		// create `PhysFrame` types from the start addresses
		frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
	}
}

unsafe impl FrameAllocator<Size4KiB> for CustomFrameAllocator
{
	fn allocate_frame(&mut self) -> Option<PhysFrame>
	{
		let frame = self.usable_frames().nth(self.next);
		self.next += 1;
		frame
	}
}
