// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// TODO write tests

/// ### Initialize a Global Allocator
///
/// Initializes a simple global allocator.
pub fn initialize()
{
	use crate::prelude::*;

	log_info!("Initializing a simple global memory allocator");

	log_debug!("Initialized allocator");
}

/// ## Simple Fixed-Block-Size Allocator
///
/// Contains an allocator that implements the fixed-block-allocation procedure.
#[allow(dead_code)]
mod fixed_block_size
{
	use crate::prelude::kernel_types::lock;
	use alloc::alloc;

	/// ### The Block Sizes to Use
	///
	/// The sizes must each be power of 2 because they are also used as
	/// the block alignment (alignments must be always powers of 2).
	const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

	/// ### The Module Allocator
	///
	/// The structure implementing the allocation algorithm of this module.
	pub struct Allocator
	{
		/// TODO
		list_heads:         [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
		/// TODO
		fallback_allocator: linked_list_allocator::Heap,
	}

	/// TODO
	struct ListNode
	{
		/// TODO
		next: Option<&'static mut ListNode>,
	}

	impl Allocator
	{
		/// TODO
		pub const fn new() -> Self
		{
			/// TODO
			const EMPTY: Option<&'static mut ListNode> = None;

			Self {
				list_heads:         [EMPTY; BLOCK_SIZES.len()],
				fallback_allocator: linked_list_allocator::Heap::empty(),
			}
		}

		/// ### Initialize the allocator with the given heap bounds.
		///
		/// This function is unsafe because the caller must guarantee that the
		/// given heap bounds are valid and that the heap is unused. This method
		/// must be called only once.
		pub unsafe fn initialize(&mut self, heap_start: usize, heap_size: usize)
		{
			self.fallback_allocator.init(heap_start, heap_size);
		}

		/// TODO
		fn fallback_allocate(&mut self, layout: alloc::Layout) -> *mut u8
		{
			match self.fallback_allocator.allocate_first_fit(layout) {
				Ok(ptr) => ptr.as_ptr(),
				Err(_) => ::core::ptr::null_mut(),
			}
		}

		/// TODO
		///
		/// Choose an appropriate block size for the given layout.
		///
		/// Returns an index into the `BLOCK_SIZES` array.
		fn list_index(layout: &alloc::Layout) -> Option<usize>
		{
			let required_block_size = layout.size().max(layout.align());
			BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
		}
	}

	unsafe impl alloc::GlobalAlloc for lock::Locked<Allocator>
	{
		unsafe fn alloc(&self, _layout: alloc::Layout) -> *mut u8
		{
			todo!();
		}

		unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::Layout)
		{
			todo!();
		}
	}
}
