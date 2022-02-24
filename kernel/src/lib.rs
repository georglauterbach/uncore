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
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]
// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(crate::library::prelude::test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]
// the following feature are still unstable and guarded
// behind feature gates that have to be unlocked
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(maybe_uninit_slice)]
#![feature(negative_impls)]
#![feature(panic_info_message)]
#![feature(type_alias_impl_trait)]

//! # The `unCORE` Operating System Kernel Library
//!
//! This is `unCORE`, an operating system kernel completely written in pure, idiomatic and
//! clean Rust. This "crate" provides the library and actual modules for the kernel.
//!
//! ## Vision
//!
//! `unCORE` is not trying to invent the wheel anew. As of now, `unCORE` is an educational
//! project that does not run real software. We want to change this in the future.
//! `unCORE` shall make use of well-known and common concepts used in _UNIX_ /
//! _GNU-Linux_. But, we acknowledge that modern software development is heavily
//! benefitting of CI pipelines, GIT platforms (such as _GitHub_) and collaboration in the
//! form of issues, pull requests, projects and other actions. While we know that mailing
//! lists work, we belief that modern software development can do better. One aspect we
//! heavily focus on is code quality. The motto here is: **We either do it right or not at
//! all**. Please also read the [conventions set by this
//! project](development.md#miscellaneous) to ensure you're up-to-date when it comes to
//! writing real code.
//!
//! ## Why Rust
//!
//! We believe that Rust is able to circumvent many of the fallacies and idiosyncrasies of
//! older programming languages like C or C++. We love C, but we also know that C brings
//! legacy problems with it. Rust on the other hands has modern packaging, out-of-the-box
//! testing, very strong guarantees due to its type system and many more benefits.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

extern crate alloc;

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

#[cfg(target_arch = "x86_64")]
#[cfg(test)]
bootloader::entry_point!(library::architectures::kernel_main);

/// ### Kernel Main Function
///
/// This is the architecture-independent main function which handles kernel setup.
pub(crate) fn kernel_main(boot_information: &boot::Information) -> !
{
	library::log::initialize(Some(log::Level::Trace));
	library::log::display_initial_information();

	prelude::log_trace!("Bootloader information:\n\n{:#?}\n", boot_information);

	#[cfg(test)]
	log_info!("Running unit-tests of 'lib.rs'");

	library::architectures::initialize();
	library::memory::initialize(boot_information);

	#[cfg(test)]
	crate::__test_runner();

	exit_kernel(kernel_types::ExitCode::Success)
}

/// ### Default Panic Handler
///
/// This function provides a very basic panic handler, that, depending
/// on whether you are running tests or not, writes an exit code and
/// does not return afterwards. Note that we do not unwind the stack.
#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { library::prelude::panic::callback(false, panic_info) }
