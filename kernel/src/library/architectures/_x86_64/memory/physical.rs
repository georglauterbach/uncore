// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::memory;

use x86_64::structures::paging;

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

impl From<x86_64::PhysAddr> for memory::PhysicalAddress
{
	fn from(address: x86_64::PhysAddr) -> Self { Self::new(address.as_u64() as usize) }
}

impl From<memory::Frame<memory::ChunkSizeDefault>> for paging::PhysFrame<paging::Size4KiB>
{
	fn from(frame: memory::Frame<memory::ChunkSizeDefault>) -> Self
	{
		Self::from_start_address(x86_64::PhysAddr::new(frame.start().into())).unwrap()
	}
}

impl From<paging::PhysFrame<paging::Size4KiB>> for memory::Frame<memory::ChunkSizeDefault>
{
	fn from(frame: paging::PhysFrame) -> Self { Self::new(frame.start_address().into()) }
}

impl memory::FrameAllocation<memory::ChunkSizeDefault> for FrameAllocator
{
	fn allocate_frame(&mut self) -> Result<memory::Frame<memory::ChunkSizeDefault>, ()>
	{
		use paging::FrameAllocator;

		if let Some(frame) = self.0.allocate_frame() {
			Ok(frame.into())
		} else {
			Err(())
		}
	}
}

impl From<usize> for memory::PhysicalAddress
{
	fn from(address_value: usize) -> Self { Self::new(address_value) }
}

impl From<u64> for memory::PhysicalAddress
{
	fn from(address_value: u64) -> Self { Self::new(address_value as usize) }
}

impl From<i64> for memory::PhysicalAddress
{
	fn from(address_value: i64) -> Self { Self::new(address_value as usize) }
}

impl From<memory::PhysicalAddress> for usize
{
	fn from(address: memory::PhysicalAddress) -> Self { address.inner() }
}

impl From<memory::PhysicalAddress> for u64
{
	fn from(address: memory::PhysicalAddress) -> Self { address.inner() as u64 }
}

impl From<memory::PhysicalAddress> for i64
{
	fn from(address: memory::PhysicalAddress) -> Self { address.inner() as i64 }
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