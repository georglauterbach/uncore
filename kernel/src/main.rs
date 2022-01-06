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
#![test_runner(test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]

//! # The `unCORE` Operating System Kernel
//!
//! This is `unCORE`, an operating system kerne completely written in
//! pure, idiomatic Rust.
//!
//! This file provides the "entrypoint" for the main binary, i.e. the
//! kernel, as well as functions for integration tests.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

/// ### Imports
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
/// not `main.rs`'s root.
///
/// Make sure to **always** use `library::` instead of `crate::lib::`
/// or `lib::` or something else.
use kernel::prelude::*;

/// ### Default Panic Handler
///
/// This function provides a very basic panic handler, that, depending
/// on whether you are running tests or not, writes an exit code and
/// does not return afterwards. Note that we do not unwind the stack.
#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }

/// ### Kernel Binary Entrypoint
///
/// This is the kernel's entry point directly called by the boot-code
/// (written in assembly). We're still in the UEFI boot services are
/// still enabled: it is our job to disable them now.
#[no_mangle]
pub extern "C" fn kernel_main(
	_multiboot2_magic_value: u32,
	_multiboot2_boot_information_pointer: u32,
) -> !
{
	#[cfg(test)]
	__test_runner();

	// hardware::init();
	// hardware::memory::init(boot_information);

	never_return()
}
