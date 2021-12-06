#![no_std]
#![no_main]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(custom_test_frameworks)]
#![test_runner(helper::test_runner)]
#![reexport_test_harness_main = "__start_tests"]

/// # Important Information
///
/// This file provides the "entrypoint" for the main binary, i.e. the
/// kernel, as well as functions for integration tests.
///
/// **The explanation for all the attributes above and more can be
/// found in `./lib.rs`.**

/// # Imports
///
/// The `kernel::library` is used here explicitly with the `use`
/// statement, and not with the `mod` statement. As `kernel::library`
/// is already used in `lib.rs`, we do not want to re-import it here
/// and possibly confuse Cargo.
///
/// The only exceptions so far is the `init()` function called at the
/// beginning of `_start`. It is called vi a`kernel::init()` which is
/// perfectly fine.
///
/// ## Macros
///
/// We will need to re-import all needed macros, as per definition
/// they reside in `crate`, which to be exact is `lib.rs`'s root and
/// **not** `main.rs`'s root.
///
/// Make sure to **always** use `library::` instead of `crate::lib::`
/// or `lib::` or something else.
use kernel::library;

/// # Entrypoint
///
/// The `_start` function is the entrypoint which is directly "called"
/// after booting. The bootloader will set up a stack and call this
/// function.
#[no_mangle]
pub extern "C" fn _start() -> !
{
	library::init();

	#[cfg(test)]
	__start_tests();

	helper::never_return()
}
