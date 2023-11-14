// SPDX-License-Identifier: GPL-3.0-or-later

//! This module is the "[prelude](https://stackoverflow.com/questions/36384840/what-is-the-prelude)".
//! It contains useful types, macros and other re-exports that provide useful for many
//! other modules.

/// Signals whether something was successfully
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum UncoreResult {
  /// If the action succeeded, this value of the enum indicates success
  Ok,
  /// If the action failed, this value of the enum indicates failure
  Err,
}

impl From<UncoreResult> for Result<(), ()> {
  fn from(value: UncoreResult) -> Self {
    match value {
      UncoreResult::Ok => Self::Ok(()),
      UncoreResult::Err => Self::Err(()),
    }
  }
}

/// If a function returns [`Result`], this macro will check whether [`Err`] is returned as
/// an instantiation and if so, will call [`panic!`].
#[macro_export]
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

/// Symbols in linker scripts need special handling to extract their values as they do not
/// have a "value" in the normal, high-level programming language sense, but their address
/// holds their value. refer to
/// [this guide](https://mcyoung.xyz/2021/06/01/linker-script/#linker-symbols).
macro_rules! transform_linker_symbol_to_value {
  ($identifier:ident) => {
    unsafe { core::ptr::addr_of!($identifier) }
  };

  (mut $identifier:ident) => {
    unsafe { core::ptr::addr_of_mut!($identifier) }
  };

  ($identifier:ident, $final_type:ty) => {
    crate::transform_linker_symbol_to_value!($identifier) as $final_type
  };

  (mut $identifier:ident, $final_type:ty) => {
    crate::transform_linker_symbol_to_value!(mut $identifier) as $final_type
  };
}
pub(crate) use transform_linker_symbol_to_value;
