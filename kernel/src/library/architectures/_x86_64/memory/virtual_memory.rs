// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::{
	*,
	memory::{
		virtual_memory,
		physical_memory,
	},
};

use x86_64::structures::paging;

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

impl<'a, S: virtual_memory::ChuckSize> virtual_memory::paging::PageAllocation<S> for PageTable<'a>
{
	fn allocate_page<FA>(&mut self)
	{
		use crate::library::memory::physical_memory::FrameAllocation;
		use x86_64::structures::paging::Mapper;

		let frame_allocator = unsafe { physical_memory::KERNEL_FRAME_ALLOCATOR.get_mut().unwrap() };

		let frame: physical_memory::Frame<virtual_memory::ChunkSizeDefault> =
			if let Ok(frame) = frame_allocator.allocate_frame() {
				frame
			} else {
				log_error!("Could not allocate frame during page allocation");
				exit_kernel(kernel_types::ExitCode::Failure);
			};

		let page: virtual_memory::paging::Page<virtual_memory::ChunkSizeDefault> =
			virtual_memory::paging::Page::new(virtual_memory::VirtualAddress::new(
				0x1234_1234 as usize,
			));

		let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;
		unsafe {
			self.0.map_to(page.into(), frame.into(), flags, &mut frame_allocator.0)
				.unwrap()
				.flush();
		}
	}
}

impl From<usize> for virtual_memory::VirtualAddress
{
	fn from(address_value: usize) -> Self { Self::new(address_value) }
}

impl From<u64> for virtual_memory::VirtualAddress
{
	fn from(address_value: u64) -> Self { Self::new(address_value as usize) }
}

impl From<i64> for virtual_memory::VirtualAddress
{
	fn from(address_value: i64) -> Self { Self::new(address_value as usize) }
}

impl From<virtual_memory::VirtualAddress> for usize
{
	fn from(address: virtual_memory::VirtualAddress) -> Self { address.inner() }
}

impl From<virtual_memory::VirtualAddress> for u64
{
	fn from(address: virtual_memory::VirtualAddress) -> Self { address.inner() as u64 }
}

impl From<virtual_memory::VirtualAddress> for i64
{
	fn from(address: virtual_memory::VirtualAddress) -> Self { address.inner() as i64 }
}

impl From<x86_64::VirtAddr> for virtual_memory::VirtualAddress
{
	fn from(address: x86_64::VirtAddr) -> Self { Self::new(address.as_u64() as usize) }
}

impl<S: virtual_memory::ChuckSize> From<virtual_memory::paging::Page<S>> for paging::Page<paging::Size4KiB>
{
	fn from(page: virtual_memory::paging::Page<S>) -> Self
	{
		Self::from_start_address(x86_64::VirtAddr::new(page.start().into())).unwrap()
	}
}

impl From<paging::Page<paging::Size4KiB>> for virtual_memory::paging::Page<virtual_memory::ChunkSizeDefault>
{
	fn from(page: paging::Page<paging::Size4KiB>) -> Self { Self::new(page.start_address().into()) }
}
