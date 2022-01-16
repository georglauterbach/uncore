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

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use kernel::{
	library,
	prelude::*,
};

#[no_mangle]
pub extern "C" fn efi_main(
	uefi_handle: uefi::Handle,
	uefi_system_table_boot: library::boot::UEFISystemTableBootTime,
) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	main(library::boot::exit_boot_services(
		uefi_handle,
		uefi_system_table_boot,
	))
}

fn main(_: library::boot::UEFIMemoryMap) -> !
{
	log_info!("This is the 'should_panic' test");

	this_test_should_panic();

	log_error!("Test did not panic but was expected to. FAILURE.");
	test::qemu::exit_with_failure();

	never_return()
}

fn this_test_should_panic()
{
	assert_eq!(0, 1);
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(true, panic_info) }
