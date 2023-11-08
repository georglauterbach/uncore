// SPDX-License-Identifier: GPL-3.0-or-later

//! TODO

/// TODO
pub mod arch;

/// TODO
pub mod log;

/// TODO
pub mod test;

/// TODO
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Condition {
  /// TODO
  Success,
  /// TODO
  Failure,
}

/// TODO
macro_rules! panic_on_error {
  ($function_name:path) => {{
    if let Err(error) = $function_name() {
      panic!("{}", error);
    }
  }};

  ($function_name:path, $($arguments:expr),*) => {{
    if let Err(error) = $function_name($($arguments),*) {
      panic!("{}", error);
    }
  }};
}
pub(crate) use panic_on_error;

/// TODO
macro_rules! transform_linker_symbol_to_value {
  ($identifier:ident) => {
    unsafe { core::ptr::addr_of!($identifier) }
  };

  ($identifier:ident, $final_type:ty) => {
    unsafe { core::ptr::addr_of!($identifier) as $final_type }
  };
}
pub(crate) use transform_linker_symbol_to_value;
