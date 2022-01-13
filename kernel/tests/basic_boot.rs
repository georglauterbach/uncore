// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

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
#![test_runner(test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use kernel::{
	library,
	prelude::*,
};

#[no_mangle]
pub fn kernel_main(
	multiboot2_bootloader_magic_value: u32,
	multiboot2_boot_information_pointer: u32,
) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	log_info!("This is the 'basic_boot' test");


	let _uefi_memory_map = library::boot::boot(
		multiboot2_bootloader_magic_value,
		multiboot2_boot_information_pointer,
	);

	__test_runner();

	never_return()
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }

#[test_case]
fn test_println()
{
	log_debug!("Test log output does not panic.");
}
