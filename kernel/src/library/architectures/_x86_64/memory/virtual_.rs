// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

use x86_64::structures::paging;

impl memory::ChuckSize for memory::ChunkSizeDefault
{
	const SIZE: usize = usize::pow(2, 12);
	const SIZE_AS_DEBUG_STRING: &'static str = "4KiB (4096Byte)";
}

impl memory::ChuckSize for memory::ChunkSizeHuge
{
	const SIZE: usize = usize::pow(2, 21);
	const SIZE_AS_DEBUG_STRING: &'static str = "2MiB (2097152Byte)";
}

impl memory::ChuckSize for memory::ChunkSizeGiant
{
	const SIZE: usize = usize::pow(2, 30);
	const SIZE_AS_DEBUG_STRING: &'static str = "1GiB (1073741824 Byte)";
}

/// ### A Page Table
///
/// This structure represent the page table used by the kernel.
pub struct PageTable<'a>(pub paging::OffsetPageTable<'a>);

impl<'a> PageTable<'a>
{
	/// ### Create a New Page Table
	///
	/// Constructs a new page table given an architecture specific page table.
	#[must_use]
	pub const fn new(page_table: paging::OffsetPageTable<'a>) -> Self { Self(page_table) }
}

impl<'a, S: memory::ChuckSize> memory::paging::PageAllocation<S> for PageTable<'a>
{
	fn allocate_page(&mut self, address: memory::VirtualAddress)
	{
		use memory::FrameAllocation;
		use x86_64::structures::paging::Mapper;

		let frame_allocator = unsafe { memory::get_frame_allocator() };

		let frame: memory::Frame<memory::ChunkSizeDefault> =
			if let Ok(frame) = frame_allocator.allocate_frame() {
				frame
			} else {
				log_error!("Could not allocate frame during page allocation");
				exit_kernel(kernel_types::ExitCode::Failure);
			};

		let page: memory::paging::Page<memory::ChunkSizeDefault> = memory::paging::Page::new(address);

		let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;
		unsafe {
			self.0.map_to(page.into(), frame.into(), flags, &mut frame_allocator.0)
				.unwrap()
				.flush();
		}
	}
}

impl From<usize> for memory::VirtualAddress
{
	fn from(address_value: usize) -> Self { Self::new(address_value) }
}

impl From<u64> for memory::VirtualAddress
{
	fn from(address_value: u64) -> Self { Self::new(address_value as usize) }
}

impl From<i64> for memory::VirtualAddress
{
	fn from(address_value: i64) -> Self { Self::new(address_value as usize) }
}

impl From<memory::VirtualAddress> for usize
{
	fn from(address: memory::VirtualAddress) -> Self { address.inner() }
}

impl From<memory::VirtualAddress> for u64
{
	fn from(address: memory::VirtualAddress) -> Self { address.inner() as u64 }
}

impl From<memory::VirtualAddress> for i64
{
	fn from(address: memory::VirtualAddress) -> Self { address.inner() as i64 }
}

impl From<x86_64::VirtAddr> for memory::VirtualAddress
{
	fn from(address: x86_64::VirtAddr) -> Self { Self::new(address.as_u64() as usize) }
}

impl<S: memory::ChuckSize> From<memory::paging::Page<S>> for paging::Page<paging::Size4KiB>
{
	fn from(page: memory::paging::Page<S>) -> Self
	{
		Self::from_start_address(x86_64::VirtAddr::new(page.start().into())).unwrap()
	}
}

impl From<paging::Page<paging::Size4KiB>> for memory::paging::Page<memory::ChunkSizeDefault>
{
	fn from(page: paging::Page<paging::Size4KiB>) -> Self { Self::new(page.start_address().into()) }
}
