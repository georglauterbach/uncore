// TODO work in progress
// ! THIS IS WIP

use x86_64::{
	structures::paging::PageTable,
	VirtAddr,
};

pub fn init(boot_information: &bootloader::BootInfo)
{
	crate::log_debug!("Boot information structure: \n{:#?}\n", boot_information);

	let phys_mem_offset = if let Some(offset) = boot_information.physical_memory_offset.into_option() {
		offset
	} else {
		0
	};

	crate::log_trace!("Physical memory offset = {}", phys_mem_offset);

	let phys_mem_offset = VirtAddr::new(phys_mem_offset);
	let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    
	for (i, entry) in l4_table.iter().enumerate() {
	    if !entry.is_unused() {
		crate::log_debug!("L4 Entry {}: {:?}", i, entry);
	    }
	}
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called
/// once to avoid aliasing `&mut` references (which is undefined
/// behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable
{
	let (level_4_table_frame, _) = x86_64::registers::control::Cr3::read();
	let phys = level_4_table_frame.start_address();
	let virt = physical_memory_offset + phys.as_u64();
	let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

	crate::log_trace!("Page table starts at virtual address {:?}", virt);

	&mut *page_table_ptr
}

// use x86_64::registers::control::Cr3;

// let (level_4_page_table, _) = Cr3::read();
// kernel::log!("Level 4 page table at: {:?}",
// level_4_page_table.start_address());

// kernel::log!("Bootloader information: {:#?}", boot_information);
