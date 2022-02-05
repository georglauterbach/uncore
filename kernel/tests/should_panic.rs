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
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//! # Kernel Panic Test
//!
//! Checks whether the panic handler works as expected.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use kernel::{
	library,
	prelude::*,
};

bootloader::entry_point!(kernel_test_main);

fn kernel_test_main(_boot_information: &'static mut bootloader::BootInfo) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	log_info!("This is the 'should_panic' test");

	this_test_should_panic();

	log_error!("Test did not panic but was expected to. FAILURE.");
	exit_kernel(kernel_types::ExitCode::Failure)
}

fn this_test_should_panic()
{
	assert_eq!(0, 1);
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic::callback(true, panic_info) }
