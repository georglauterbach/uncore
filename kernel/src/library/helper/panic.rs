// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use ::core::panic::PanicInfo;

use crate::prelude::*;

/// ### Panic Handler when Running Tests that Should Not Panic
///
/// This function is marked for conditional compilation, and
/// is used when running the custom tests suite.
#[cfg(test)]
#[inline]
fn __default_panic(_panic_info: &PanicInfo) -> !
{
	log_error!("Last test did not finish. FAILURE.");
	log_error!("Received panic");

	never_return()
}

/// ### Panic Handler when not Running Tests
///
/// This function is marked for conditional compilation, and
/// is used when running the binary natively, i.e. not the
/// tests.
#[cfg(not(test))]
#[inline]
fn __default_panic(panic_info: &PanicInfo) -> !
{
	log_error!(
		"Received panic (reason: {:?}) - aborting",
		panic_info
			.message()
			.unwrap_or(&format_args!("no message provided"))
	);

	test::qemu::exit_with_failure();
	never_return()
}

/// ### Panic Handler that Should Panic
///
/// This function provides a panic handler that should panic and that
/// will therefore show success.
#[inline]
fn __should_panic(_panic_info: &PanicInfo) -> !
{
	log_info!("Received expected panic - nice");

	test::qemu::exit_with_success();
	never_return()
}

/// ### Callback Panic Handler
///
/// This function exists because we want to write integration tests
/// for which certain tests should panic. Without this function, we'd
/// need to repeat the panic code for each integration test. If a test
/// should panic, set `should_panic` to `true` and the code will take
/// care of the rest. Now, only small code repetition is necessary:
///
///
/// If tests SHOULD NOT panic, write this in your application
///
/// ``` rust
/// #[panic_handler]
/// pub fn panic(panic_info: &::core::panic::PanicInfo) -> !
/// {
///         kernel::panic_callback(false, panic_info)
/// }
/// ```
///
/// If tests SHOULD panic, write this in your application
///
/// ``` rust
/// #[panic_handler]
/// pub fn panic(panic_info: &::core::panic::PanicInfo) -> !
/// {
///         kernel::panic_callback(true, panic_info)
/// }
/// ```
#[inline]
#[allow(clippy::module_name_repetitions)]
pub fn panic_callback(should_panic: bool, panic_info: &PanicInfo) -> !
{
	if should_panic {
		__should_panic(panic_info);
	} else {
		__default_panic(panic_info);
	}
}
