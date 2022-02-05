// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

/// ### Default Panic Handler
///
/// This is the default panic handler, which uses conditional compilation to be used in
/// both tests and when running normally.
#[inline]
fn default_panic(panic_info: &::core::panic::PanicInfo) -> !
{
	#[cfg(not(test))]
	log_error!(
		"Received panic (reason: {:?}) - aborting",
		panic_info
			.message()
			.unwrap_or(&format_args!("no message provided"))
	);

	#[cfg(test)]
	log_error!(
		"Received panic (reason: {:?})",
		panic_info
			.message()
			.unwrap_or(&format_args!("no message provided"))
	);
	#[cfg(test)]
	log_error!("Last test did not finish (successfully). FAILURE.");

	exit_kernel(kernel_types::ExitCode::Failure)
}

/// ### Panic Handler that Should Panic
///
/// This function provides a panic handler that should panic and that
/// will therefore show success.
#[inline]
fn should_panic(_panic_info: &::core::panic::PanicInfo) -> !
{
	log_info!("Received expected panic - nice");
	exit_kernel(kernel_types::ExitCode::Success)
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
///         kernel::panic::callback(false, panic_info)
/// }
/// ```
///
/// If tests SHOULD panic, write this in your application
///
/// ``` rust
/// #[panic_handler]
/// pub fn panic(panic_info: &::core::panic::PanicInfo) -> !
/// {
///         kernel::panic::callback(true, panic_info)
/// }
/// ```
#[inline]
pub fn callback(callback_should_panic: bool, panic_info: &::core::panic::PanicInfo) -> !
{
	if callback_should_panic {
		should_panic(panic_info);
	} else {
		default_panic(panic_info);
	}
}
