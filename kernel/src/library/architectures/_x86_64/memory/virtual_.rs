// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use x86_64::structures::paging::{
	self,
	Mapper,
};

impl memory::ChunkSize for memory::ChunkSizeDefault
{
	const SIZE: usize = usize::pow(2, 12);
	const SIZE_AS_DEBUG_STRING: &'static str = "4KiB (4096Byte)";
}

impl memory::ChunkSize for memory::ChunkSizeHuge
{
	const SIZE: usize = usize::pow(2, 21);
	const SIZE_AS_DEBUG_STRING: &'static str = "2MiB (2097152Byte)";
}

impl memory::ChunkSize for memory::ChunkSizeGiant
{
	const SIZE: usize = usize::pow(2, 30);
	const SIZE_AS_DEBUG_STRING: &'static str = "1GiB (1073741824 Byte)";
}

/// ### A Page Table
///
/// This structure represent the page table used by the kernel.
#[derive(Debug)]
pub struct PageTable<'a>(pub paging::OffsetPageTable<'a>);

impl<'a> PageTable<'a>
{
	/// ### Create a New Page Table
	///
	/// Constructs a new page table given an architecture specific page table.
	#[must_use]
	pub const fn new(page_table: paging::OffsetPageTable<'a>) -> Self { Self(page_table) }
}

impl<'a> memory::paging::PageAllocation for PageTable<'a>
{
	fn allocate_page(
		&mut self,
		address: memory::VirtualAddress,
	) -> Result<(), kernel_types::errors::VirtualMemory>
	{
		use memory::FrameAllocation;

		let frame_allocator = unsafe { memory::get_frame_allocator() };

		let frame: memory::Frame<memory::ChunkSizeDefault> = match frame_allocator.allocate_frame() {
			Ok(frame) => frame,
			Err(error) => {
				log_error!(
					"Could not allocate frame during page allocation (error: {:?}",
					error
				);
				exit_kernel(kernel_types::ExitCode::Failure);
			},
		};

		let frame = frame.try_into()?;
		let page = paging::Page::containing_address(address.into());
		let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;

		unsafe {
			match self.0.map_to(page, frame, flags, &mut frame_allocator.0) {
				Ok(flush) => {
					flush.flush();
					Ok(())
				},
				Err(error) => {
					log_error!("Page mapping resulted in error (error: {:?}", error);
					Err(kernel_types::errors::VirtualMemory::PageMappingError)
				},
			}
		}
	}

	fn deallocate_page(
		&mut self,
		address: memory::VirtualAddress,
	) -> Result<(), kernel_types::errors::VirtualMemory>
	{
		let page: paging::Page<paging::Size4KiB> = paging::Page::containing_address(address.into());
		match self.0.unmap(page) {
			Ok((_frame, flush)) => {
				flush.flush();
				// TODO we should probably deallocate the frame as well...
				Ok(())
			},
			// TODO
			Err(_error) => {
				// use paging::mapper::UnmapError;
				use kernel_types::errors::VirtualMemory;
				Err(VirtualMemory::PageDeallocationPageWasNotMapped)
			},
		}
	}
}

impl From<x86_64::VirtAddr> for memory::VirtualAddress
{
	#[allow(clippy::cast_possible_truncation)]
	fn from(address: x86_64::VirtAddr) -> Self { Self::new(address.as_u64() as usize) }
}

impl From<memory::VirtualAddress> for x86_64::VirtAddr
{
	fn from(address: memory::VirtualAddress) -> Self { Self::new(address.inner() as u64) }
}
