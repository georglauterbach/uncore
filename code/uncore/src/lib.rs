// SPDX-License-Identifier: GPL-3.0-or-later

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]
// #![allow(clippy::multiple_crate_versions)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(crate::library::test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]

//! # The `unCORE` Operating System Kernel Library
//!
//! This is `unCORE`, an operating system kernel completely written in pure, idiomatic and
//! clean Rust. This "crate" provides the library and actual modules for the kernel.

// ? UNSTABLE FEATURES
// ? ---------------------------------------------------------------------

// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// Allows reading the message of a call to `panic!()`.
#![feature(panic_info_message)]

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

/// The `alloc` crate enables the kernel to dynamically allocate memory. The module
/// [`mod@crate::library::mem::heap`] implements the corresponding allocators.
extern crate alloc;

/// ### The Core Library
///
/// This module has been created to give the kernel source code a
/// well-defined structure and layout. The `library` module is used as
/// the child of the `src/lib.rs` "crate", not of `src/main.rs`. This
/// is important, and we are not allowed to mix them up.
mod library;

/// Public re-exports that ought to be used by `main.rs`, by integration tests, or inside
/// the kernel with `crate::`.
pub use library::{
  arch,
  test,
  prelude::*,
};

/// This function can be described as the kernel's "main" function. It usually runs after
/// architecture-specific setup functions have run.
pub fn setup_kernel(hart: usize) {
  if hart == 0 {
    library::log::initialize();
    library::log::display_initial_information();
  }

  log::info!("Running on HART {}", hart);

  if hart == 0 {
    library::mem::heap::Heap::initialize();
  }
}
