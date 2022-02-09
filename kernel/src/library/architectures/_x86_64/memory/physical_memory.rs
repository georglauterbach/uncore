// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::memory::{
	physical_memory,
	virtual_memory,
};

/// ### A Frame Allocator
///
/// This structure enables frame allocation (i.e. handling of physical addresses and
/// "selection" of physical memory regions).
pub struct FrameAllocator(pub frame_allocation::BootInfoFrameAllocator);

impl FrameAllocator
{
	/// ### Create a New Frame Allocator
	///
	/// This function creates a new instance of a frame allocator for the `x86_64`
	/// architecture.
	#[must_use]
	pub const fn new(allocator: frame_allocation::BootInfoFrameAllocator) -> Self { Self(allocator) }
}

impl From<x86_64::PhysAddr> for physical_memory::PhysicalAddress
{
	fn from(address: x86_64::PhysAddr) -> Self { Self::new(address.as_u64() as usize) }
}

impl<S: virtual_memory::ChuckSize> From<physical_memory::Frame<S>>
	for x86_64::structures::paging::PhysFrame<x86_64::structures::paging::Size4KiB>
{
	fn from(frame: physical_memory::Frame<S>) -> Self
	{
		Self::from_start_address(x86_64::PhysAddr::new(frame.start().into())).unwrap()
	}
}

impl<S: virtual_memory::ChuckSize>
	From<x86_64::structures::paging::PhysFrame<x86_64::structures::paging::Size4KiB>>
	for physical_memory::Frame<S>
{
	fn from(frame: x86_64::structures::paging::PhysFrame) -> Self
	{
		Self::new(frame.start_address().into())
	}
}

impl<S: virtual_memory::ChuckSize> physical_memory::FrameAllocation<S> for FrameAllocator
{
	fn allocate_frame(&mut self) -> Result<physical_memory::Frame<S>, ()>
	{
		use x86_64::structures::paging::FrameAllocator;

		if let Some(frame) = self.0.allocate_frame() {
			Ok(frame.into())
		} else {
			Err(())
		}
	}
}

impl From<usize> for physical_memory::PhysicalAddress
{
	fn from(address_value: usize) -> Self { Self::new(address_value) }
}

impl From<u64> for physical_memory::PhysicalAddress
{
	fn from(address_value: u64) -> Self { Self::new(address_value as usize) }
}

impl From<i64> for physical_memory::PhysicalAddress
{
	fn from(address_value: i64) -> Self { Self::new(address_value as usize) }
}

impl From<physical_memory::PhysicalAddress> for usize
{
	fn from(address: physical_memory::PhysicalAddress) -> Self { address.inner() }
}

impl From<physical_memory::PhysicalAddress> for u64
{
	fn from(address: physical_memory::PhysicalAddress) -> Self { address.inner() as u64 }
}

impl From<physical_memory::PhysicalAddress> for i64
{
	fn from(address: physical_memory::PhysicalAddress) -> Self { address.inner() as i64 }
}

/// ## Physical Frame Allocation
///
/// This module contains the structures for allocation and reservation of physical frames.
pub mod frame_allocation
{
	use bootloader::boot_info;
	use x86_64::structures::paging;

	/// The Kernel's Frame Allocator for `x86_64`
	///
	/// This structure can use the information given to it by the [`bootloader`]
	/// crate.
	pub struct BootInfoFrameAllocator
	{
		/// The map of used / unused frames.
		memory_map: &'static boot_info::MemoryRegions,
		/// The next frame to be allocated.
		next:       usize,
	}

	impl BootInfoFrameAllocator
	{
		/// ### Create a New Frame Allocator
		///
		/// Create a frame allocator from the passed memory map.
		///
		/// This function is unsafe because the caller must guarantee that the
		/// passed memory map is valid. The main requirement is that all frames
		/// that are marked as `USABLE` in it are really unused.
		pub const unsafe fn new(memory_map: &'static boot_info::MemoryRegions) -> Self
		{
			Self { memory_map, next: 0 }
		}

		/// ### Return the Next Usable Frame(s)
		///
		/// Returns an iterator over the usable frames specified in the memory
		/// map.
		fn usable_frames(&self) -> impl Iterator<Item = paging::PhysFrame>
		{
			// get usable regions from memory map
			let regions = self.memory_map.iter();
			let usable_regions =
				regions.filter(|r| r.kind == boot_info::MemoryRegionKind::Usable);
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
