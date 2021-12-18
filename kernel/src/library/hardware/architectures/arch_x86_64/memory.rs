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
pub fn init(boot_information: &bootloader::BootInfo)
{
	let _offset_page_table = unsafe { create_offset_page_table(boot_information) };
}

/// ### Initialize a new `OffsetPageTable`
///
/// This function crates the page table structure for `uncORE`. As the
/// complete physical memory is mapped by the bootloader, we cam just
/// use an offset to calculate the first level 4 page table.
///
/// #### `unsafe`
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
	let physical_memory_mapping_offset = boot_information
		.physical_memory_offset
		.into_option()
		.map_or_else(
			|| {
				crate::log_warning!(
					"Physical memory offset is non-existent (defaulting to 0)"
				);
				VirtAddr::new(0)
			},
			VirtAddr::new,
		);

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
pub fn _create_example_mapping(
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
