// SPDX-License-Identifier: GPL-3.0-or-later

//! Contains architecture-specific information about the kernel-heap, like its starting
//! address and its size.

extern "C" {
  static mut __heap__start: u8;
  static __heap__size: u8;
}

/// Returns the starting address of the kernel heap.
#[must_use]
pub fn get_start() -> *mut u8 { crate::transform_linker_symbol_to_value!(mut __heap__start) }

/// Returns the size of the kernel heap.
#[must_use]
pub fn get_size() -> usize { crate::transform_linker_symbol_to_value!(__heap__size, usize) }
