// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use x86_64::structures::paging;
// use crate::memory::{virtual_memory, physical_memory};

/// TODO
pub struct PageTable<'a>(Option<paging::OffsetPageTable<'a>>);

impl<'a> PageTable<'a>
{
	/// TODO
	#[must_use]
	pub const fn new(page_table: Option<paging::OffsetPageTable<'a>>) -> Self { Self(page_table) }
}

// impl<'a> virtual_memory::paging::PageAllocation for PageTable<'a>
// {
// 	fn allocate_page<FA>(&mut self, _frame_allocator: FA)
// 	where
// 		FA: memory::FrameAllocation,
// 	{
// 		unimplemented!()
// 	}
// }
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
