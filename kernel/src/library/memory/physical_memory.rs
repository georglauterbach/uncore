// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library::architectures::memory::physical_memory as architecture_physical_memory;

use ::core::marker;

/// ### Global Kernel Frame Allocator
///
/// Structure containing the kernel's global frame allocator.
pub static mut KERNEL_FRAME_ALLOCATOR: spin::once::Once<architecture_physical_memory::FrameAllocator> =
	spin::once::Once::new();

/// ### A Physical Address Abstraction
///
/// This is an opaque wrapper type that contains the address as its first type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress
{
	/// ### Create a New Physical Address
	///
	/// Constructs a new physical address.
	pub fn new(address: impl Into<usize>) -> Self { Self(address.into()) }

	/// ### Get the Inner Value
	///
	/// Returns the inner value, i.e. content that is wrapped by this type.
	pub fn inner(&self) -> usize { self.0 }
}

impl ::core::ops::Add for PhysicalAddress
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output { Self::new(self.inner() + rhs.inner()) }
}

impl ::core::ops::Add<usize> for PhysicalAddress
{
	type Output = Self;

	fn add(self, rhs: usize) -> Self::Output { Self::new(self.inner() + rhs) }
}

impl ::core::ops::Add<u64> for PhysicalAddress
{
	type Output = Self;

	fn add(self, rhs: u64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
}

impl ::core::ops::Add<i64> for PhysicalAddress
{
	type Output = Self;

	fn add(self, rhs: i64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
}

impl ::core::ops::Sub for PhysicalAddress
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output { Self::new(self.inner() - rhs.inner()) }
}

impl ::core::ops::Sub<usize> for PhysicalAddress
{
	type Output = Self;

	fn sub(self, rhs: usize) -> Self::Output { Self::new(self.inner() - rhs) }
}

impl ::core::ops::Sub<u64> for PhysicalAddress
{
	type Output = Self;

	fn sub(self, rhs: u64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
}

impl ::core::ops::Sub<i64> for PhysicalAddress
{
	type Output = Self;

	fn sub(self, rhs: i64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
}

/// ### Representation of a Page
///
/// This structure holds the information necessary to represent a memory frame with a
/// given chunk size.
pub struct Frame<S: super::virtual_memory::ChuckSize = super::virtual_memory::ChunkSizeDefault>
{
	/// Where the frame starts in physical memory.
	start_address: PhysicalAddress,
	/// How big the physical frame is.
	size:          marker::PhantomData<S>,
}

impl<S: super::virtual_memory::ChuckSize> Frame<S>
{
	/// ### Create a New Frame
	///
	/// Creates a new physical frame instance.
	pub fn new(start_address: PhysicalAddress) -> Self
	{
		Self {
			start_address,
			size: marker::PhantomData,
		}
	}

	/// ### Start Address of a Frame
	///
	/// Returns the starts address of the given frame.
	pub fn start(&self) -> PhysicalAddress { self.start_address }
}

/// ### Capability of Allocating Frames
///
/// This traits shows that a type can frames.
pub trait FrameAllocation<S: super::virtual_memory::ChuckSize>
{
	/// ### Allocate a Single Frame
	///
	/// The method with which a single frame is allocated.
	fn allocate_frame(&mut self) -> Result<Frame<S>, ()>;
}
