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
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::broken_intra_doc_links)]
// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(kernel::prelude::test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]
#![feature(type_alias_impl_trait)]

//! # The `unCORE` Operating System Kernel
//!
//! This is `unCORE`, an operating system kerne completely written in
//! pure, idiomatic Rust.
//!
//! This file provides the "entrypoint" for the main binary, i.e. the
//! kernel, as well as functions for integration tests.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use kernel::{
	library,
	prelude::*,
};

/// ### UEFI Entrypoint
///
/// This function is called before [`main`] is called. It handled
/// initialization for logging exiting UEFI boot services.
#[no_mangle]
pub extern "C" fn efi_main(
	uefi_handle: uefi::Handle,
	uefi_system_table_boot: library::boot::UEFISystemTableBootTime,
) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	// https://github.com/rust-osdev/bootloader/blob/main/src/bin/uefi.rs#L37
	kernel_main(library::boot::exit_boot_services(
		uefi_handle,
		uefi_system_table_boot,
	))
}

/// ### Kernel Main Entrypoint
///
/// This is the kernel's entry point directly called by the boot-code
/// (written in assembly). We're still in the UEFI boot services are
/// still enabled: it is our job to disable them now.
fn kernel_main(_uefi_memory_map: library::boot::UEFIMemoryMap) -> !
{
	#[cfg(test)]
	__test_runner();

	log_info!("Starting architecture specific initialization");
	library::architectures::cpu::initialize();

	#[cfg(target_arch = "x86_64")]
	test::qemu::exit_with_success();

	never_return()
}

/// ### Default Panic Handler
///
/// This function provides a very basic panic handler, that, depending
/// on whether you are running tests or not, writes an exit code and
/// does not return afterwards. Note that we do not unwind the stack.
#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }

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
