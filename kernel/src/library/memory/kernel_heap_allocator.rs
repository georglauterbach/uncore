// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

/// ### The Global Kernel Allocator
///
/// This structure implements the [`alloc::GlobalAlloc`] trait to allocator kernel heap
/// memory.
#[global_allocator]
static ALLOCATOR: kernel_types::lock::Locked<fixed_block_size::Allocator> =
	kernel_types::lock::Locked::from(fixed_block_size::Allocator::new());

/// ### Kernel Heap Error Handler
///
/// This function shows errors during kernel heap allocation. In a nutshell, it panics
/// with an appropriate message...
#[alloc_error_handler]
fn allocator_error_handler(layout: ::alloc::alloc::Layout) -> !
{
	panic!("allocation error (layout: {:?})", layout);
}

/// ### Initialize a Global Allocator
///
/// Initializes a simple global kernel heap memory allocator.
pub fn initialize()
{
	log_info!("Initializing a simple global memory allocator");
	unsafe {
		ALLOCATOR
			.lock()
			.initialize(super::KERNEL_HEAP_START, super::KERNEL_HEAP_SIZE);
	}
	log_debug!("Initialized allocator");
}

/// ## Simple Fixed-Block-Size Allocator
///
/// Contains an allocator that implements the fixed-block-allocation procedure.
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
	/// The structure implementing the allocation algorithm of this module. See
	/// <https://os.phil-opp.com/allocator-designs/#fixed-size-block-allocator>
	pub struct Allocator
	{
		/// The head pointers for each block size.
		list_heads:         [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
		/// A fallback allocator used when the default allocator does not work
		/// (for an arbitrary reason).
		fallback_allocator: linked_list_allocator::Heap,
	}

	/// ### A List Node
	///
	/// A simple node that holds a "pointer" to the next free node (for the same block
	/// size).
	struct ListNode
	{
		/// The next "pointer" pointing to the next node.
		next: Option<&'static mut ListNode>,
	}

	impl Allocator
	{
		/// ### Create a New Allocator Instance
		///
		/// This constant function returns a new allocator instance.
		pub const fn new() -> Self
		{
			/// Circumventing a weakness of the Rust `const` evaluator. This
			/// constant is logic-wise not actually needed.
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

		/// ### Fallback Allocation
		///
		/// If for some reason, the default allocator fails, the fallback
		/// allocator takes over.
		fn allocate_with_fallback_allocator(&mut self, layout: alloc::Layout) -> *mut u8
		{
			crate::prelude::log_warning!(
				"Had to allocator kernel heap memory with the fallback allocator"
			);
			match self.fallback_allocator.allocate_first_fit(layout) {
				Ok(ptr) => ptr.as_ptr(),
				Err(_) => ::core::ptr::null_mut(),
			}
		}

		/// ### Choose the Correct Block Size
		///
		/// Choose an appropriate block size for the given layout. Returns an
		/// index into the `BLOCK_SIZES` array.
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
