// SPDX-License-Identifier: GPL-3.0-or-later

//! This module contains all functionality required for unit-testing `unCORE`. It also
//! contains the entrypoints of the unit-test binaries for every architecture.

/// ### Streamlining Testing
///
/// This trait provides the tests runner with the ability to `.run`
/// tests. This is done for all functions in the `impl` block, so they
/// can be "parsed" to reduce boilerplate code.
pub trait Testable {
  /// ### Run Tests
  ///
  /// The `run` function will literally just execute the
  /// function it contains, as `Testable` is implemented for all
  /// generics that implement `Fn()`.
  fn run(&self);
}

impl<T> Testable for T
where
  T: Fn(),
{
  fn run(&self) {
    log::debug!("Testing {}", ::core::any::type_name::<Self>());
    self();
    log::trace!("Most recent test passed");
  }
}

/// ### A (Very) Simple Test Runner Implementation
///
/// This function is registered as the tests runner when executing
/// Cargo test's unit tests.
///
/// It will just execute all functions marked with `#[test_case]` one
/// by one.
#[cfg(test)]
pub fn runner(tests: &[&dyn Testable]) {
  log::info!("Running unit-tests");

  for test in tests {
    test.run();
  }

  log::info!("Last test finished successfully");
  crate::arch::exit_kernel(crate::UncoreResult::Ok);
}

/// ### Sanity Check
///
/// This tests is just here for sanity's sake to make
/// sure tests behave correctly at the most basic level.
#[test_case]
fn trivial_assertion() {
  const ONE: u8 = 1;
  assert_eq!(1, ONE);
  assert_eq!(ONE, 1);
}

/// The unit-test entry point of `lib.rs`. This function is run when unit tests for
/// `lib.rs` are run.
#[cfg(all(target_arch = "riscv64", test))]
#[riscv_rt::entry]
fn riscv64_entry(hart: usize) -> ! {
  crate::arch::initialize(hart);
  crate::setup_kernel(hart);
  crate::__test_runner();
  crate::arch::exit_kernel(crate::UncoreResult::Ok);
}
