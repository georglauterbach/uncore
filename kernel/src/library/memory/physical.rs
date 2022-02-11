// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;
use crate::library::architectures::memory as architecture_memory;

/// ### Global Kernel Frame Allocator
///
/// Structure containing the kernel's global frame allocator.
pub(super) static mut KERNEL_FRAME_ALLOCATOR: spin::once::Once<architecture_memory::FrameAllocator> =
	spin::once::Once::new();

/// ### Acquire A Mutable Frame Allocator Reference
///
/// Tis function returns a mutable reference to the kernel's frame allocator needed during
/// page allocation.
///
/// #### Safety
///
/// This function is `unsafe` as it returns a mutable reference from a `static mut`
/// variable. This function will panic if the allocator is not initialized.
pub(crate) unsafe fn get_frame_allocator() -> &'static mut architecture_memory::FrameAllocator
{
	KERNEL_FRAME_ALLOCATOR
		.get_mut()
		.expect("Could not acquire frame allocator (is it initialized?)")
}

/// ### Representation of a Page
///
/// This structure holds the information necessary to represent a memory frame with a
/// given chunk size.
pub struct Frame<S: super::ChunkSize = super::ChunkSizeDefault>
{
	/// Where the frame starts in physical memory.
	start_address: memory::PhysicalAddress,
	/// How big the physical frame is.
	size:          ::core::marker::PhantomData<S>,
}

impl<S: super::ChunkSize> Frame<S>
{
	/// ### Create a New Frame
	///
	/// Creates a new physical frame instance.
	pub fn new(start_address: memory::PhysicalAddress) -> Self
	{
		Self {
			start_address,
			size: ::core::marker::PhantomData,
		}
	}

	/// ### Start Address of a Frame
	///
	/// Returns the starts address of the given frame.
	pub fn start(&self) -> memory::PhysicalAddress { self.start_address }
}

/// ### Capability of Allocating Frames
///
/// This traits shows that a type can frames.
pub trait FrameAllocation<S: super::ChunkSize>
{
	/// ### Allocate a Single Frame
	///
	/// The method with which a single frame is allocated.
	fn allocate_frame(&mut self) -> Result<Frame<S>, ()>;
}
