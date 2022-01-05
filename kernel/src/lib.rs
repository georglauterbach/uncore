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
// Since the `x86-interrupt` calling convention is still unstable, we
// have to opt-in.
#![feature(abi_x86_interrupt)]
// Since retrieving the message during a call to `panic!` is
// still unstable, we have to opt-in.
#![feature(panic_info_message)]
// Checking the target ABI is still experimental
// and subject to change.
#![feature(cfg_target_abi)]
#![feature(global_asm)]

//! # The `unCORE` Operating System Kernel
//!
//! This is `unCORE`, an operating system kerne completely written in
//! pure, idiomatic Rust.
//!
//! This file provides the library and modules for the actual binary.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

// global_asm!(include_str!("library/boot/arch/x86_64/boot.S"),
// options(att_syntax)); global_asm!(include_str!("library/boot/arch/
// x86_64/multiboot2.S"), options(att_syntax));

/// ### The Core Library Path
///
/// This module has been created to give the kernel source code a
/// well-defined structure and layout. The `library` module is used as
/// the child of the `src/lib.rs` "crate", not of `src/main.rs`. This
/// is important, and we are not allowed to mix them up.
pub mod library;
pub use library::prelude;

use library::{
	// hardware,
	// log,
	prelude::panic_callback,
};

/// ### Default Panic Handler
///
/// This function provides a very basic panic handler, that, depending
/// on whether you are running tests or not, writes an exit code and
/// does not return afterwards. Note that we do not unwind the stack.
#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }

// * x86_64
// * -----------------------------

// use bootloader as x86_64_bootloader;

// /// ### Kernel Library Testing Entrypoint (`x86_64`)
// ///
// /// This is the kernel's entry point called after the bootloader
// has /// finished its setup. It is kept short on purpose. The
// /// `library::init()` function takes care of initialization. This
// /// function is effectively run only during unit tests.
// #[cfg(target_arch = "x86_64")]
// #[no_mangle]
// pub extern "C" fn _start(boot_information: &'static mut
// x86_64_bootloader::BootInfo) -> ! {
// 	log::init(Some(log::Level::Trace), boot_information);

// 	hardware::init();
// 	hardware::memory::init(boot_information);

// 	log_info!("Kernel initialization finished");

// 	#[cfg(test)]
// 	crate::__test_runner();

// 	never_return()
// }
