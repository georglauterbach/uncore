// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

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

//! # The `unCORE` Boot-Image Creation
//!
//! This workspace member takes care of creating a bootable image by utilizing the
//! `bootloader` crate for `x86_64`.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS / VARIABLES
// ? ---------------------------------------------------------------------

/// ### Kernel Binary Location
///
/// Holds the location of the compiled kernel binary in your file system.
const KERNEL_BINARY: Option<&str> = option_env!("KERNEL_BINARY");

/// ### Repository Root Directory
///
/// Holds the location of the repository root directory in your file system.
const ROOT_DIRECTORY: Option<&str> = option_env!("ROOT_DIRECTORY");

/// ## Architecture Specific Code
///
/// Holds _architecture dependent_ code.
mod architectures;

/// ## Binary Arguments
///
/// Manages the arguments this binary can take.
mod arguments;

/// ### Just `main`
fn main()
{
	workspace_helper::logger::initialize(log::Level::Info);
	let arguments = arguments::Arguments::new();

	architectures::link_with_bootloader(&arguments);
}
