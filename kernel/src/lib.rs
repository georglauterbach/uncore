// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// If we're testing, there is no main function, but a custom
// entrypoint `_start`.
#![cfg_attr(test, no_main)]
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
#![test_runner(crate::library::prelude::test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]
// Since the `x86-interrupt` calling convention is still unstable,
// we have to opt-in.
#![feature(abi_x86_interrupt)]
// Checking the target ABI is still experimental
// and subject to change.
#![feature(cfg_target_abi)]
// Dereferencing raw mutable pointers in constant functions is still
// unstable.
#![feature(const_mut_refs)]
// Since retrieving the message during a call to `panic!` is
// still unstable, we have to opt-in.
#![feature(panic_info_message)]
// Defining `type = impl ...` has not yet been stabilized, so
// we need to open this feature gate.
#![feature(type_alias_impl_trait)]

//! # The `unCORE` Operating System Kernel
//!
//! This is `unCORE`, an operating system kerne completely written in
//! pure, idiomatic Rust.
//!
//! This file provides the library and modules for the actual binary.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

/// ### The Core Library
///
/// This module has been created to give the kernel source code a
/// well-defined structure and layout. The `library` module is used as
/// the child of the `src/lib.rs` "crate", not of `src/main.rs`. This
/// is important, and we are not allowed to mix them up.
pub mod library;

/// ### Re-Exporting the Prelude
///
/// The `prelude` module shall be accessible from `crate::` (or
/// `kernel::` in case of `main.rs`).
pub use library::prelude;
use library::prelude::*;

/// ### Kernel Library Testing - UEFI Entrypoint
///
/// This function is called before [`main`] is called. It handled
/// initialization for logging exiting UEFI boot services. For
/// `lib.rs`, this is the entrypoint for tests.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn efi_main(
	uefi_handle: uefi::Handle,
	uefi_system_table_boot: library::boot::UEFISystemTableBootTime,
) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	kernel_main(&library::boot::exit_boot_services(
		uefi_handle,
		uefi_system_table_boot,
	))
}

/// ### Kernel Library Testing - Kernel Main Entrypoint
///
/// This is the kernel's entry point called after the bootloader has
/// finished its setup. It is kept short on purpose. The
/// `library::init()` function takes care of initialization. This
/// function is effectively run only during unit tests.
#[cfg(test)]
fn kernel_main(_uefi_memory_map: &library::boot::UEFIMemoryMap) -> !
{
	log_info!("Running unit-tests of 'lib.rs'");

	log_info!("Starting architecture specific initialization");
	library::architectures::cpu::initialize();

	#[cfg(test)]
	__test_runner();

	#[cfg(target_arch = "x86_64")]
	test::qemu::exit_with_success();

	never_return()
}

/// ### Default Panic Handler
///
/// This function provides a very basic panic handler, that, depending
/// on whether you are running tests or not, writes an exit code and
/// does not return afterwards. Note that we do not unwind the stack.
#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }
