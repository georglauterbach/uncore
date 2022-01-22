// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::kernel_types::lock;

/// TODO
#[global_allocator]
static ALLOCATOR: lock::Locked<fixed_block_size::Allocator> =
	lock::Locked::from(fixed_block_size::Allocator::new());

/// TODO
#[alloc_error_handler]
fn allocator_error_handler(layout: ::alloc::alloc::Layout) -> !
{
	panic!("allocation error (layout: {:?})", layout);
}

/// ### Initialize a Global Allocator
///
/// Initializes a simple global allocator.
pub fn initialize()
{
	// use crate::prelude::*;

	// log_info!("Initializing a simple global memory allocator");
	// unsafe { ALLOCATOR.lock().initialize(); }
	// log_debug!("Initialized allocator");
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
		fn allocate_with_fallback_allocator(&mut self, layout: alloc::Layout) -> *mut u8
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
		unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8
		{
			let mut allocator = self.lock();
			if let Some(index) = Allocator::list_index(&layout) {
				if let Some(node) = allocator.list_heads[index].take() {
					allocator.list_heads[index] = node.next.take();
					(node as *mut ListNode).cast::<u8>()
				} else {
					// no block exists in list => allocate new block
					let block_size = BLOCK_SIZES[index];
					// only works if all block sizes are a power of 2
					let block_align = block_size;
					let layout = alloc::Layout::from_size_align(block_size, block_align)
						.unwrap();
					allocator.allocate_with_fallback_allocator(layout)
				}
			} else {
				allocator.allocate_with_fallback_allocator(layout)
			}
		}

		unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout)
		{
			let mut allocator = self.lock();
			if let Some(index) = Allocator::list_index(&layout) {
				let new_node = ListNode {
					next: allocator.list_heads[index].take(),
				};

				// verify that block has size and alignment required for storing node
				assert!(::core::mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
				assert!(::core::mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

				#[allow(clippy::cast_ptr_alignment)]
				let new_node_ptr = ptr.cast::<ListNode>();

				new_node_ptr.write(new_node);
				allocator.list_heads[index] = Some(&mut *new_node_ptr);
			} else {
				let ptr = ::core::ptr::NonNull::new(ptr).unwrap();
				allocator.fallback_allocator.deallocate(ptr, layout);
			}
		}
	}
}
