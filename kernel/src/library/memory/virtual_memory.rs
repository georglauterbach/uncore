// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// use spin::once::Once;

use crate::library::architectures::memory::virtual_memory;

/// ### Kernel Page Table
///
/// Represents the global page table held by the kernel for demand paging.
pub static mut KERNEL_PAGE_TABLE: virtual_memory::PageTable = virtual_memory::PageTable::new(None);
impl ::core::ops::Add for VirtualAddress
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output { Self::new(self.inner() + rhs.inner()) }
}

impl ::core::ops::Add<usize> for VirtualAddress
{
	type Output = Self;

	fn add(self, rhs: usize) -> Self::Output { Self::new(self.inner() + rhs) }
}

impl ::core::ops::Add<u64> for VirtualAddress
{
	type Output = Self;

	fn add(self, rhs: u64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
}

impl ::core::ops::Add<i64> for VirtualAddress
{
	type Output = Self;

	fn add(self, rhs: i64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
}

impl ::core::ops::Sub for VirtualAddress
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output { Self::new(self.inner() - rhs.inner()) }
}

impl ::core::ops::Sub<usize> for VirtualAddress
{
	type Output = Self;

	fn sub(self, rhs: usize) -> Self::Output { Self::new(self.inner() - rhs) }
}

impl ::core::ops::Sub<u64> for VirtualAddress
{
	type Output = Self;

	fn sub(self, rhs: u64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
}

impl ::core::ops::Sub<i64> for VirtualAddress
{
	type Output = Self;

	fn sub(self, rhs: i64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
}

/// ### Determine Page Size
///
/// This struct can is used to abstract over all available page sizes of a system.
pub trait ChuckSize: Copy
{
	/// Page size is bytes.
	const SIZE: usize;

	/// Page size as string for debug purposes.
	const SIZE_AS_DEBUG_STRING: &'static str;
}

/// ### Architecture Default Page Size
///
/// Represents the default page size for an architecture. On `x86_64` the size is 4096
/// Bytes.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeDefault;

impl ChuckSize for ChunkSizeDefault
{
	const SIZE: usize = usize::pow(2, 12);
	const SIZE_AS_DEBUG_STRING: &'static str = "4KiB (4096Byte)";
}

impl ChunkSizeDefault
{
	/// ### Default Size
	///
	/// Returns the size of the default page size ([`ChunkSizeDefault::SIZE`]).
	pub(crate) const fn size() -> usize { Self::SIZE }
}

/// ### Architecture's Big Pages
///
/// The bigger-than-default pages.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeHuge;

impl ChuckSize for ChunkSizeHuge
{
	const SIZE: usize = usize::pow(2, 21);
	const SIZE_AS_DEBUG_STRING: &'static str = "2MiB (2097152Byte)";
}

/// ### Architecture's Biggest Pages
///
/// This is the biggest page size available for an architecture.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeGiant;

impl ChuckSize for ChunkSizeGiant
{
	const SIZE: usize = usize::pow(2, 30);
	const SIZE_AS_DEBUG_STRING: &'static str = "1GiB (1073741824 Byte)";
}

/// ## Demand Paging
///
/// Contains the needed types for proper demand paging.
pub mod paging
{
	use ::core::marker;

	/// ### A Virtual Memory Address
	///
	/// A simple wrapper for a virtual address.
	#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct VirtualAddress(usize);

	impl VirtualAddress
	{
		/// TODO
		pub fn new(address: impl Into<usize>) -> Self { Self(address.into()) }
	}

	impl ::core::convert::From<usize> for VirtualAddress
	{
		fn from(start_address: usize) -> Self { Self(start_address) }
	}

	/// ### Representation of a Page
	///
	/// This structs holds the information of a single page.
	struct Page<S: super::ChuckSize>
	{
		start_address: VirtualAddress,
		size:          marker::PhantomData<S>,
	}

	impl<S: super::ChuckSize> Page<S>
	{
		/// ### Create a New Page
		///
		/// This function creates a new page.
		pub fn new(start_address: VirtualAddress) -> Self
		{
			Self {
				start_address,
				size: marker::PhantomData,
			}
		}
	}

	/// ### Capability of Allocating Pages
	///
	/// This traits shows that a type can allocate pages with the help of a frame
	/// allocator.
	pub trait PageAllocation<S: super::ChuckSize>
	{
		/// ### Allocate a Single Page
		/// 
		/// The method with which a single page is allocated.
		fn allocate_page<FA>(&mut self, frame_allocator: FA)
		where
			FA: super::super::physical_memory::FrameAllocation<S>;
	}
}
