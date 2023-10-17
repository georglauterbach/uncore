// SPDX-License-Identifier: GPL-3.0-or-later

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
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]
// the following feature are still unstable and guarded
// behind feature gates that have to be unlocked
#![feature(is_some_with)]

//! # The `unCORE` Helper Binary
//!
//! This crate (implicitly created by `main.rs`) wraps building, running and testing the
//! kernel.
//!
//! ## Proper Invocation
//!
//! It should be invoked by `Just`, but if you want to do it manually, run
//!
//! ```console
//! $ source scripts/init.sh
//! $ set +e
//! $ cargo run --package helper -- --help
//! ```

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

/// ## A Fancy Name for Different Tasks
///
/// Contains the modules that wrap the different tasks of building, running and testing
/// `unCORE`.
mod dispatch;

/// ### Entrypoint
///
/// This is a simple, nice, beautiful `main` function, right?
fn main()
{
	use clap::Parser;

	// parse arguments
	let arguments = dispatch::arguments::Arguments::parse();
	log::trace!("Arguments:\n{:#?}\n", arguments);

	// set up log
	let log_level = arguments.get_log_level();
	helper::environment::set_log_level(helper::logger::Logger::level_to_string(&log_level));
	helper::logger::initialize(Some(log_level));

	// run the specified command
	helper::build::set_target(&arguments.target);
	arguments.execute_command();
}
