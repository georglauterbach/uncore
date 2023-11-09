// SPDX-License-Identifier: GPL-3.0-or-later

//! TODO

use crate::library;

extern "C" {
  static __heap__start: u8;
  static __heap__size: u8;
}

#[allow(unused)]
/// TODO
pub struct Heap {
  /// TODO
  start: *const u8,
  /// TODO
  size:  usize,
}

impl Heap {
  /// TODO
  fn _new() -> Self {
    Self {
      start: crate::library::transform_linker_symbol_to_value!(__heap__start),
      size:  crate::library::transform_linker_symbol_to_value!(__heap__size, usize),
    }
  }
}

/// TODO
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern "C" fn DefaultHandler() { crate::arch::exit_kernel(library::Condition::Failure); }
