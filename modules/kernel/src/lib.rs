#![no_std]
#![cfg_attr(test, no_main)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(custom_test_frameworks)]
#![test_runner(helper::test::test_runner)]
#![reexport_test_harness_main = "__start_tests"]

/// # The Library
///
/// ## Kernel Parameters
///
/// As this is an absolutely freestanding project, we cannot rely on
/// the standard-library, and we will need to use our own entrypoint,
/// not `main()`.
///
/// Therefore,
///
/// ``` RUST
/// #![no_std]
/// #![no_main]
/// ```
///
/// are used.
///
/// ## Linting Targets
///
/// A very strict set of rules is employed to guarantee clear, robust
/// and idiomatic code.
///
/// Therefore,
///
/// ``` RUST
/// #![deny(clippy::all)]
/// #![deny(clippy::nursery)]
/// #![deny(clippy::pedantic)]
/// ```
///
/// are used.
///
/// ## Custom Tests
///
/// As we cannot use the standard-library, we will need to use our own
/// tests framework.
///
/// Therefore,
///
/// ``` RUST
/// #![feature(custom_test_frameworks)]
/// #![test_runner(kernel::library::tests::test_runner)]
/// #![reexport_test_harness_main = "__start_tests"]
/// ```
///
/// are used.
///
/// ## Cargo Auto-Detection
///
/// Cargo's auto-detection of library files is turned on. Therefore,
/// `src/lib.rs` is automatically detected by Cargo as a
/// (freestanding) library. We need to define some code segments
/// twice, here as well as an in `src/main.rs` as this file is tested
/// by Cargo separately.
///
/// This file can then be used in integration tests as well.
///
/// ## Other Features
///
/// Since the `x86-interrupt` calling convention is still unstable, we
/// use
///
/// ``` RUST
/// #![feature(abi_x86_interrupt)]
/// ```
///
/// to use it nevertheless.

/// # The Core Library Path
///
/// This module has been created to give the kernel source code a
/// well-defined structure and layout. The `library` module is used as
/// the child of the `src/lib.rs` "crate", not of `src/main.rs`. This
/// is important, and we are not allowed to mix them up.
pub mod library;

/// # Entrypoint
///
/// The `_start` function is the entrypoint which is directly "called"
/// after booting. The bootloader will set up a stack and call this
/// function. Note that this is only running when we're running tests.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !
{
	library::init();
	__start_tests();
	helper::never_return()
}
