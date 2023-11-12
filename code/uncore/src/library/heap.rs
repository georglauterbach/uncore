// SPDX-License-Identifier: GPL-3.0-or-later

//! This module holds all functionality required for working with the kernel heap.

/// This data structure represents the kernel heap.
#[allow(clippy::module_name_repetitions)]
pub struct _Heap {
  /// The starting address of the heap
  pub start: *const u8,
  /// The size of the heap
  pub size:  usize,
}

impl _Heap {
  /// Constructs a new instance of [`_Heap`].
  fn _new() -> Self {
    Self {
      start: crate::arch::heap::get_start(),
      size:  crate::arch::heap::get_size(),
    }
  }
}
