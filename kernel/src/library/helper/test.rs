// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

/// ### Are We Running Tests?
///
/// Can be used to get information about whether tests are run or not.
#[allow(dead_code)]
#[cfg(test)]
pub const IS_TEST: bool = true;

/// ### Are We Running Tests?
///
/// Can be used to get information about whether tests are run or not.
#[allow(dead_code)]
#[cfg(not(test))]
pub const IS_TEST: bool = false;

/// ### Streamlining Testing
///
/// This trait provides the tests runner with the ability to `.run`
/// tests. This is done for all functions in the `impl` block, so they
/// can be "parsed" to reduce boilerplate code.
pub trait Testable
{
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
	fn run(&self)
	{
		log_debug!("Testing {}", ::core::any::type_name::<Self>());
		self();
		log_trace!("Most recent test passed");
	}
}

/// ### A (Very) Simple Test Runner Implementation
///
/// This function is registered as the tests runner when executing
/// Cargo test's unit tests.
///
/// It will just execute all functions marked with `#[test_case]` one
/// by one.
#[allow(clippy::module_name_repetitions)]
pub fn runner(tests: &[&dyn Testable])
{
	log_info!("Starting tests");

	for test in tests {
		test.run();
	}

	log_info!("Last test finished successfully");
	qemu::exit_with_success();
}

/// ### Sanity Check
///
/// This tests is just here for sanity's sake to make
/// sure tests behave correctly at the most basic level.
#[test_case]
fn trivial_assertion()
{
	const ONE: u8 = 1;
	assert_eq!(1, ONE);
	assert_eq!(ONE, 1);
}

/// ## QEMU Abstractions
///
/// Contains helpers for running the kernel with QEMU.
pub mod qemu
{

	/// ### Write An Exit Code
	///
	/// Writes to the `0xF4` port the correct bytes that indicate
	/// either success or failure.
	#[inline]
	fn exit(success: bool)
	{
		use qemu_exit::QEMUExit;

		#[cfg(target_arch = "x86_64")]
		let qemu_exit_handle = qemu_exit::X86::new(0xF4, 0x3);

		if success {
			qemu_exit_handle.exit_success();
		} else {
			qemu_exit_handle.exit_failure();
		}
	}

	/// ### Exit QEMU With Success
	///
	/// Write a success exit code for QEMU to recognize and exit.
	#[allow(dead_code)]
	#[inline]
	pub fn exit_with_success() { exit(true); }

	/// ### Exit QEMU Without Success
	///
	/// Write a failure exit code for QEMU to recognize and exit.
	#[allow(dead_code)]
	#[inline]
	pub fn exit_with_failure() { exit(false); }
}
