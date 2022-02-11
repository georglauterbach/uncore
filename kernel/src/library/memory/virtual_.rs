// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library::architectures::memory::virtual_ as architecture_virtual_memory;
use crate::prelude::*;

/// ### Kernel Page Table
///
/// Represents the global page table held by the kernel for demand paging.
pub static mut KERNEL_PAGE_TABLE: spin::Mutex<spin::once::Once<architecture_virtual_memory::PageTable>> =
	spin::Mutex::new(spin::Once::new());

/// ### Allocate a Single Page
///
/// This function takes a virtual address and allocates a single page for it.
pub fn allocate_page(address: VirtualAddress)
{
	use paging::PageAllocation;
	unsafe { KERNEL_PAGE_TABLE.lock() }
		.get_mut()
		.expect("Could not acquire kernel page table")
		.allocate_page(address);
}

/// ### Allocate Multiple Pages At Once
///
/// This function allocates a page for the virtual address given and `page_count` pages
/// afterwards.
pub fn allocate_range(start: impl Into<VirtualAddress>, page_count: usize) -> usize
{
	let address = start.into();
	log_debug!(
		"Allocating range at {:?} for {} default-sized pages",
		address,
		page_count
	);

	let page_range: paging::PageRange<ChunkSizeDefault> =
		paging::PageRange::new(paging::Page::new(address), page_count);

	let size = page_range.size_in_bytes();
	for page in page_range {
		allocate_page(page.start());
	}

	size
}

/// ### A Virtual Memory Address
///
/// A simple wrapper for a virtual address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualAddress(usize);

impl VirtualAddress
{
	/// ### Create a New Virtual Address
	///
	/// Takes a
	pub fn new(address: usize) -> Self { Self(address) }

	/// ### Get the Inner Value
	///
	/// Returns the inner value, i.e. content that is wrapped by this type.
	pub fn inner(&self) -> usize { self.0 }
}

impl From<usize> for VirtualAddress
{
	fn from(address_value: usize) -> Self { Self::new(address_value) }
}

impl From<VirtualAddress> for usize
{
	fn from(address: VirtualAddress) -> Self { address.inner() }
}

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
pub trait ChunkSize: Copy
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

/// ### Architecture's Biggest Pages
///
/// This is the biggest page size available for an architecture.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeGiant;

/// ## Demand Paging
///
/// Contains the needed types for proper demand paging.
pub mod paging
{
	/// ### Representation of a Page
	///
	/// This structs holds the information of a single page.
	#[derive(Debug, Copy, Clone)]
	pub struct Page<S: super::ChunkSize>
	{
		start_address: super::VirtualAddress,
		size:          ::core::marker::PhantomData<S>,
	}

	impl<S: super::ChunkSize> Page<S>
	{
		/// ### Create a New Page
		///
		/// This function creates a new page.
		pub fn new(start_address: super::VirtualAddress) -> Self
		{
			Self {
				start_address,
				size: ::core::marker::PhantomData,
			}
		}

		/// ### Start Address of a Page
		///
		/// Returns the starts address of the given page.
		pub fn start(&self) -> super::VirtualAddress { self.start_address }
	}

	impl<S: super::ChunkSize> ::core::cmp::PartialEq for Page<S>
	{
		fn eq(&self, other: &Self) -> bool { self.start_address == other.start_address }
	}

	impl<S: super::ChunkSize> ::core::cmp::Eq for Page<S> {}

	impl<S: super::ChunkSize> ::core::cmp::PartialOrd for Page<S>
	{
		fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
		{
			self.start_address.partial_cmp(&other.start_address)
		}
	}

	impl<S: super::ChunkSize> ::core::cmp::Ord for Page<S>
	{
		fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.start().cmp(&other.start()) }
	}

	impl<S: super::ChunkSize> ::core::ops::Add<u64> for Page<S>
	{
		type Output = Self;

		fn add(self, rhs: u64) -> Self::Output { Page::new(self.start() + rhs as usize * S::SIZE) }
	}

	impl<S: super::ChunkSize> ::core::ops::AddAssign<u64> for Page<S>
	{
		fn add_assign(&mut self, rhs: u64) { *self = *self + rhs; }
	}

	/// ### Capability of Allocating Pages
	///
	/// This traits shows that a type can allocate pages with the help of a frame
	/// allocator.
	pub trait PageAllocation
	{
		/// ### Allocate a Single Page
		///
		/// The method with which a single page is allocated.
		fn allocate_page(&mut self, address: super::VirtualAddress);
	}

	/// ### Representation of Multiple Pages
	///
	/// Represents a (inclusive) range of pages.
	#[derive(Debug, Copy, Clone)]
	pub struct PageRange<S: super::ChunkSize = super::ChunkSizeDefault>
	{
		/// The first page in the range.
		start: Page<S>,
		/// The last page (inclusive) in the range.
		end:   Page<S>,
		// The size of the range in bytes.
		size:  usize,
	}

	impl<S: super::ChunkSize> PageRange<S>
	{
		/// ### Create a new Range of Pages
		///
		/// This function takes the start address and the amount of pages one
		/// needs. This constructor does not take a specific `end` page to avoid
		/// common mistakes such as the end being before the start, etc.
		pub fn new(start: Page<S>, page_count: usize) -> Self
		{
			assert_ne!(
				page_count, 0,
				"Page count in page range was 0 which is not allowed"
			);
			let size = (page_count * S::SIZE) - 1;
			let end = Page::new(start.start() + size);
			Self { start, end, size }
		}

		/// ### The Page Range's Size
		/// 
		/// Returns the Size of the page range in bytes.
		pub fn size_in_bytes(&self) -> usize { self.size }

		/// ### Number of Pages
		/// 
		/// Returns the number of pages the range contains.
		pub fn page_count(&self) -> usize
		{
			(self.size + 1) / S::SIZE
		}
	}

	impl<S: super::ChunkSize> Iterator for PageRange<S>
	{
		type Item = Page<S>;

		fn next(&mut self) -> Option<Self::Item>
		{
			if self.start <= self.end {
				let page = self.start;
				self.start += 1;
				Some(page)
			} else {
				None
			}
		}
	}
}
