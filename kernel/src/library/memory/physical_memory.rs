// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library::architectures::memory::physical_memory;

use ::core::marker;

/// TODO
pub static mut KERNEL_FRAME_ALLOCATOR: physical_memory::FrameAllocator =
	physical_memory::FrameAllocator::new(None);

/// TODO
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress
{
	/// TODO
	pub fn new(address: usize) -> Self { Self(address) }

	/// TODO
	pub fn as_u64(&self) -> u64
	{
		self.0 as u64
	}
}

/// TODO
#[allow(dead_code)]
pub struct Frame<S: super::virtual_memory::ChuckSize = super::virtual_memory::ChunkSizeDefault>
{
	/// TODO
	pub start_address: PhysicalAddress,
	/// TODO
	size:          marker::PhantomData<S>,
}

impl<S: super::virtual_memory::ChuckSize> Frame<S>
{
	/// TODO
	pub fn new(start_address: PhysicalAddress) -> Self
	{
		Self {
			start_address,
			size: marker::PhantomData,
		}
	}
}

/// TODO
pub trait FrameAllocation<S: super::virtual_memory::ChuckSize>
{
	/// TODO
	fn allocate_frame(&mut self) -> Result<Frame<S>, FrameAllocationError>;
}

/// TODO
#[derive(Debug, Copy, Clone)]
pub enum FrameAllocationError
{
	/// TODO
	FrameAllocatorNotInitialized,
	/// TODO
	GenericCouldNotAllocateFrame,
}
