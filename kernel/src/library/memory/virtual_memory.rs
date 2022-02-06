// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library::{
	architectures,
	prelude::*,
};

pub static mut KERNEL_PAGE_TABLE: architectures::memory::PageTable =
	architectures::memory::PageTable::new(None);

pub static mut KERNEL_FRAME_ALLOCATOR: architectures::memory::FrameAllocator =
	architectures::memory::FrameAllocator::new(None);

/// ### Virtual Memory Initialization
///
/// This function takes care of setting up virtual memory properly. It calls the
/// architecture specific setup routines and then
///
/// TODO abstracts over them with kernel specific types, i.e. architecture agnostic ones.
pub fn initialize(boot_information: &boot::Information)
{
	log_info!("Initializing virtual memory");

	let (kernel_page_table, kernel_frame_allocator) =
		architectures::memory::initialize(boot_information.0);
	unsafe {
		KERNEL_PAGE_TABLE = architectures::memory::PageTable::new(Some(kernel_page_table));
		KERNEL_FRAME_ALLOCATOR =
			architectures::memory::FrameAllocator::new(Some(kernel_frame_allocator));
	}

	log_info!("Finished initializing virtual memory");
}

#[allow(dead_code)]
struct Frame;

#[allow(dead_code)]
struct Page;

/// TODO
pub(crate) trait PageAllocation
{
	/// TODO
	fn allocate_page<FA>(&mut self, frame_allocator: FA)
	where
		FA: FrameAllocation;
}

/// TODO
pub(crate) trait FrameAllocation
{
	/// TODO
	fn allocate_frame(&mut self) -> Result<(), ()>;
}
