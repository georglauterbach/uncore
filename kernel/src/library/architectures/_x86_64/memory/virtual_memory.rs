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
