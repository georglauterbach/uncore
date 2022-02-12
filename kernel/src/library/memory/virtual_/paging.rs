// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

/// ### Representation of a Page
///
/// This structs holds the information of a single page.
#[derive(Debug, Copy, Clone)]
pub struct Page<S: memory::ChunkSize>
{
	/// The (virtual) start address of the page. This address is an alignment of the
	/// address that was given to create the page.
	start_address: memory::VirtualAddress,
	/// The size of the page in bytes.
	size:          ::core::marker::PhantomData<S>,
}

impl<S: memory::ChunkSize> Page<S>
{
	/// ### Create a New Page
	///
	/// This function creates a new page. **The start address is always aligned**
	/// before the page is created.
	#[must_use]
	pub fn new(mut start_address: memory::VirtualAddress) -> Self
	{
		start_address.align_down(S::SIZE);
		Self {
			start_address,
			size: ::core::marker::PhantomData,
		}
	}

	/// ### Start Address of a Page
	///
	/// Returns the starts address of the given page.
	#[must_use]
	pub fn start(&self) -> memory::VirtualAddress { self.start_address }
}

impl<S: memory::ChunkSize> ::core::cmp::PartialEq for Page<S>
{
	fn eq(&self, other: &Self) -> bool { self.start_address == other.start_address }
}

impl<S: memory::ChunkSize> ::core::cmp::Eq for Page<S> {}

impl<S: memory::ChunkSize> ::core::cmp::PartialOrd for Page<S>
{
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
	{
		self.start_address.partial_cmp(&other.start_address)
	}
}

impl<S: memory::ChunkSize> ::core::cmp::Ord for Page<S>
{
	fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.start().cmp(&other.start()) }
}

impl<S: memory::ChunkSize> ::core::ops::Add<u64> for Page<S>
{
	type Output = Self;

	#[allow(clippy::cast_possible_truncation)]
	fn add(self, rhs: u64) -> Self::Output { Self::new(self.start() + rhs as usize * S::SIZE) }
}

impl<S: memory::ChunkSize> ::core::ops::AddAssign<u64> for Page<S>
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
	///
	/// #### Errors
	///
	/// Page allocation can suffer from various issues, such as address alignment,
	/// issues during frame allocation or mapping problems. These errors are
	/// propagated to the caller.
	fn allocate_page(
		&mut self,
		address: memory::VirtualAddress,
	) -> Result<(), kernel_types::errors::VirtualMemory>;
}

/// ### Representation of Multiple Pages
///
/// Represents a (inclusive) range of pages.
#[derive(Debug, Copy, Clone)]
pub struct PageRange<S: memory::ChunkSize = memory::ChunkSizeDefault>
{
	/// The first page in the range.
	start: Page<S>,
	/// The last page (inclusive) in the range.
	end:   Page<S>,
	/// The size of the range in bytes.
	size:  usize,
}

impl<S: memory::ChunkSize> PageRange<S>
{
	/// ### Create a new Range of Pages
	///
	/// This function takes the start address and the amount of pages one
	/// needs. This constructor does not take a specific `end` page to avoid
	/// common mistakes such as the end being before the start, etc.
	///
	/// #### Panics
	///
	/// This function will [`panic!`] if
	///
	/// 1. `page_count` is zero
	#[must_use]
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

	/// ### Get the First Page
	///
	/// Returns a copy of the first page.
	fn start(&self) -> Page<S> { self.start }

	/// ### Get the Last Page
	///
	/// Returns a copy of the last page.
	fn end(&self) -> Page<S> { self.end }

	/// ### The Page Range's Size
	///
	/// Returns the Size of the page range in bytes.
	#[must_use]
	pub fn size_in_bytes(&self) -> usize { self.size }

	/// ### Number of Pages
	///
	/// Returns the number of pages the range contains.
	#[must_use]
	pub fn page_count(&self) -> usize { (self.size + 1) / S::SIZE }
}

impl<S: memory::ChunkSize> memory::ChunkSize for Page<S>
{
	const SIZE: usize = S::SIZE;
	const SIZE_AS_DEBUG_STRING: &'static str = S::SIZE_AS_DEBUG_STRING;
}

impl<S: memory::ChunkSize> IntoIterator for PageRange<S>
{
	type IntoIter = PageRangeIntoIterator<S>;
	type Item = Page<S>;

	fn into_iter(self) -> Self::IntoIter { self.into() }
}

/// ### [`Iterator`] for a [`Copy`] Type
///
/// See <https://rust-lang.github.io/rust-clippy/master/index.html#copy_iterator>.
/// Since [`PageRange`] is [`Copy`], we explicitly implement [`IntoIterator`], and not
/// [`Iterator`]. Then [`PageRangeIntoIterator`] implements [`Iterator`].
#[derive(Debug)]
pub struct PageRangeIntoIterator<S: memory::ChunkSize>
{
	/// The first page in the range.
	start: Page<S>,
	/// The last page (inclusive) in the range.
	end:   Page<S>,
}

impl<S: memory::ChunkSize> !Copy for PageRangeIntoIterator<S> {}

impl<S: memory::ChunkSize> From<PageRange<S>> for PageRangeIntoIterator<S>
{
	fn from(page_range: PageRange<S>) -> Self
	{
		Self {
			start: page_range.start(),
			end:   page_range.end(),
		}
	}
}

impl<S: memory::ChunkSize> Iterator for PageRangeIntoIterator<S>
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

/// ### Allocate a Single Page
///
/// This function takes a virtual address and allocates a single page for it.
///
/// #### Errors
///
/// If there is an issue during allocation, it is propagated upwards to the caller of this
/// function who can then decide what to do. The possible errors can be seen in
/// [`crate::prelude::kernel_types::errors::VirtualMemory`].
pub fn allocate_page(address: memory::VirtualAddress) -> Result<(), kernel_types::errors::VirtualMemory>
{
	unsafe { super::KERNEL_PAGE_TABLE.lock() }
		.get_mut()
		.expect("Could not acquire kernel page table")
		.allocate_page(address)
}

/// ### Allocate Multiple Pages At Once
///
/// This function allocates a page for the virtual address given and `page_count` pages
/// afterwards.
///
/// #### Errors
///
/// If there is an issue during allocation, it is propagated upwards to the caller of this
/// function who can then decide what to do. The possible errors can be seen in
/// [`crate::prelude::kernel_types::errors::VirtualMemory`].
pub fn allocate_range(
	start: impl Into<memory::VirtualAddress>,
	page_count: usize,
) -> Result<usize, kernel_types::errors::VirtualMemory>
{
	let address = start.into();
	log_trace!(
		"Allocating range at {:?} for {} default-sized pages",
		address,
		page_count
	);

	let page_range: PageRange<memory::ChunkSizeDefault> = PageRange::new(Page::new(address), page_count);

	let size = page_range.size_in_bytes();
	for page in page_range {
		allocate_page(page.start())?;
	}

	Ok(size)
}
