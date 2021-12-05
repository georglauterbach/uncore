#![no_std]
#![cfg_attr(test,no_main)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::core_lib::tests::testing::test_runner)]
#![reexport_test_harness_main = "__start_tests"]

/// # The Library
///
/// This file provides the "entrypoint" for the main binary, i.e. the kernel,
/// as well as functions for integration tests.
///
/// ## Kernel Parameters
///
/// As this is an absolutely freestanding project,
/// we cannot rely on the standard-library, and we
/// will need to use our own entrypoint, not `main()`.
///
/// Therefore,
///
/// ``` RUST
/// #![no_std]
/// #![cfg_attr(tests, no_main)]
/// ```
///
/// are used. The second attribute indicates the use of `no_main` when we are
/// executing tests.
///
/// ## Linting Targets
///
/// A very strict set of rules is employed to guarantee
/// clear, robust and idiomatic code.
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
/// As we cannot use the standard-library, we will
/// need to use our own tests framework.
///
/// Therefore,
///
/// ``` RUST
/// #![feature(custom_test_frameworks)]
/// #![test_runner(core_lib::testing::test_runner)]
/// #![reexport_test_harness_main = "__start_tests"]
/// ```
///
/// are used.
///
/// ## Cargo Auto-Detection
///
/// Cargo's auto-detection of library files is turned on.
///
/// Therefore, `src/lib.rs` is automatically detected by Cargo
/// as a (freestanding) library. We need to define some
/// code segments twice, here as well as an in `src/main.rs` as this file is
/// tested by Cargo separately.
///
/// This file can then be used in integration tests as well.
///
/// ## Other Features
///
/// Since the `x86-interrupt` calling convention is still unstable,
/// we use
///
/// ``` RUST
/// #![feature(abi_x86_interrupt)]
/// ```
///
/// to use it nevertheless.

/// # The Core Library Path
///
/// This module has been created to give the kernel source
/// code a well-defined structure and layout. The `core_lib` module
/// is used as the child of the `src/lib.rs` "crate", not of `src/main.rs`.
/// This is important, and we are not allowed to mix them up.
pub mod core_lib;

/// # Entrypoint
///
/// The `_start` function is the entrypoint which is directly "called"
/// after booting. The bootloader will set up a stack and call this
/// function.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !
{
	init();
	__start_tests();
	core_lib::misc::helper::__never_return()
}

/// # Panic Handler
///
/// This function uses a conditionally compiled function
/// depending on whether running the kernel or the tests
/// suite.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { core_lib::misc::panic::panic(info) }

/// # Global Initialization
///
/// This function takes care of initialization of registers,
/// global state and values.
pub fn init()
{
	core_lib::hw::io::write::print_init();

	core_lib::hw::init::run();
}
