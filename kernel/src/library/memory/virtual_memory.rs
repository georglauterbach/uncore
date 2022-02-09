// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library::{
	architectures::{
		self,
		memory::{
			physical_memory,
			virtual_memory,
		},
	},
	prelude::*,
};

/// ### Kernel Page Table
///
/// Represents the global page table held by the kernel for demand paging.
pub static mut KERNEL_PAGE_TABLE: virtual_memory::PageTable = virtual_memory::PageTable::new(None);

/// ### Virtual Memory Initialization
///
/// This function takes care of setting up virtual memory properly. It calls the
/// architecture specific setup routines and then abstracts over them with kernel specific
/// types, i.e. architecture agnostic ones.
pub fn initialize(boot_information: &boot::Information)
{
	log_debug!("Initializing virtual memory");

	let (mut kernel_page_table, kernel_frame_allocator) =
		architectures::memory::initialize(boot_information.0);

		
		unsafe {
			// KERNEL_PAGE_TABLE = virtual_memory::PageTable::new(Some(kernel_page_table));
			super::physical_memory::KERNEL_FRAME_ALLOCATOR =
			physical_memory::FrameAllocator::new(Some(kernel_frame_allocator));
		}
	
		architectures::memory::initialize_kernel_heap( &mut kernel_page_table);

	log_debug!("Finished initializing virtual memory");
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
/// Represents the default page size for an architecture. On `x86_64` the size is 4096 Bytes.
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
		/// TODO
		fn allocate_page<FA>(&mut self, frame_allocator: FA)
		where
			FA: super::super::physical_memory::FrameAllocation<S>;
	}
}
