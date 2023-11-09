// SPDX-License-Identifier: GPL-3.0-or-later

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf) .
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::all)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::private_intra_doc_links)]
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
// TODO
#![feature(panic_info_message)]

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

// extern crate alloc;

/// ### The Core Library
///
/// This module has been created to give the kernel source code a
/// well-defined structure and layout. The `library` module is used as
/// the child of the `src/lib.rs` "crate", not of `src/main.rs`. This
/// is important, and we are not allowed to mix them up.
mod library;

/// TODO
pub use library::{
  arch,
  test,
  Condition,
};

/// TODO
#[cfg(all(target_arch = "riscv64", test))]
#[riscv_rt::entry]
fn riscv64_entry() -> ! {
  crate::arch::initialize();
  setup_kernel();
  crate::__test_runner();
  arch::exit_kernel(library::Condition::Success);
}

/// TODO
pub fn setup_kernel() {
  library::log::initialize();
  library::log::display_initial_information();
}
