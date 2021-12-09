// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
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
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::broken_intra_doc_links)]
// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(crate::test_runner)]

//! # The unCORE Operating System Kernel Helper Function Collection
//!
//! This crate provides functions all other workspace members use for
//! general purpose tasks and requirements, such as not returning or
//! needing a test framework.
//!
//! Since we cannot have and do not want cyclic dependencies between
//! our workspace members, we outsource generic functions that all
//! workspace members need into this crate.
//!
//! Only the most generic functions shall reside in this crate.

// ? MODULES AND EXPORTS
// ? ---------------------------------------------------------------------

/// ## Miscellaneous Helpers
///
/// Provides various of the most generic helper functions, such as
/// `never_return()`.
mod miscellaneous;

/// ## Provides the API for Panicking
///
/// This module provides the implementation for the panic macro and
/// panic behavior.
mod panic;

/// ## Providing Support for Tests
///
/// This module provides the implementation to run tests. This
/// includes unit-tests as well as integration tests.
mod test;

pub use miscellaneous::never_return;
pub use panic::panic;
pub use test::test_runner;
pub use test::Testable;

// ? GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------
