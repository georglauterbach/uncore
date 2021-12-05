#![no_std]
#![cfg_attr(test, no_main)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "__start_tests"]

/// # Miscellaneous Helpers
///
/// Provides the most generic helper functions, such as
/// `__never_return()`. These are explicitly re-exported
/// and renamed in the process.
mod miscellaneous;

pub use miscellaneous::__never_return as never_return;
pub use miscellaneous::__panic as panic;

/// # Provides the API for Panicking
///
/// This module provides the implementation for the panic macro and
/// panic behavior.
pub(crate) mod panic;

/// # QEMU Abstractions
///
/// Contains helpers for QEMU.
pub mod qemu;

/// # Providing Support for Tests
///
/// This module provides the implementation to run tests. This
/// includes unit-tests as well as integration tests.
pub mod test;
