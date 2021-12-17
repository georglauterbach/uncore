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
// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(library::test_runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]

use kernel::library::{
	self,
	helper::log,
};

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn _start(boot_information: &'static mut bootloader::BootInfo) -> !
{
	log::set_log_level(log::Level::Trace);
	kernel::log!("Running an integration test.");
	library::init(boot_information);

	__test_runner();

	kernel::log_error!("Test did not panic but was expected to. FAILURE.");
	kernel::library::helper::qemu::exit_with_failure();

	library::never_return()
}

#[test_case]
fn this_test_should_panic()
{
	assert_eq!(0, 1);
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { library::panic_callback(true, panic_info) }
