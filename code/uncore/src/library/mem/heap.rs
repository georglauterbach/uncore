// SPDX-License-Identifier: GPL-3.0-or-later

//! This module holds all functionality required for working with the kernel heap.

/// This is the global kernel heap allocator. It implements
/// [`core::alloc::GlobalAlloc`].
///
/// To manage the heap, we use the [`linked_list_allocator`] crate. This is an easy yet
/// performant allocator for `#![no_std]` binaries like our kernel.
#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

/// Checks whether [`Heap::initialize`] has been called before.
static mut INIT_WAS_CALLED: bool = false;

/// This data structure represents the kernel heap.
#[allow(clippy::module_name_repetitions)]
pub struct Heap {
  /// The starting address of the heap
  start: *mut u8,
  /// The size of the heap
  size:  usize,
}

impl Heap {
  /// Constructs a new instance of [`Heap`].
  fn new() -> Self {
    Self {
      start: crate::arch::heap::get_start(),
      size:  crate::arch::heap::get_size(),
    }
  }

  /// Initialize the kernel heap by providing the allocator [`ALLOCATOR`] with a start
  /// address and a size.
  pub fn initialize() {
    assert!(
      unsafe { !INIT_WAS_CALLED },
      "called library/mem/heap.rs:initialize more than once"
    );

    unsafe {
      INIT_WAS_CALLED = true;
    }

    log::info!("Initializing kernel heap");
    let heap = Self::new();

    unsafe {
      ALLOCATOR.lock().init(heap.start, heap.size);
    }
  }
}
