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
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::broken_intra_doc_links)]

//! # The `unCORE` Test Runner Integration
//!
//! This workspace member enabled running unit- and integration tests
//! seamlessly. This test runner should only be invoked by the script
//! `test_kernel.sh` under `scripts/`.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

/// ## Provides Logging Functionality
///
/// A shameless copy of the kernel logging implementation.
mod logger;

use std::{
	path,
	process,
	time,
};

/// ### Compile Time Constant for Repository Root
///
/// When running the test runner, it is assumed this variable is set.
/// If not, we try to fall back to using `pwd`.
const ROOT_DIRECTORY: Option<&str> = option_env!("ROOT_DIRECTORY");

/// ### Individual Test Run Timeout
///
/// The time in seconds any individual test has before being
/// terminated.
const TIMEOUT: u64 = 10;

/// # Entrypoint
///
/// This is a simple, nice, beautiful `main` function, right?
fn main()
{
	logger::init(log::Level::Info);
	log::info!("Started test runner");

	// skip executable name
	let mut arguments = std::env::args().skip(1);

	let kernel_test_binary_path_string = arguments.next().map_or_else(
		|| {
			log::error!("No path to the kernel binary provided.");
			process::exit(1);
		},
		|path| path,
	);

	let kernel_test_binary_path = path::PathBuf::from(kernel_test_binary_path_string.clone());
	let kernel_test_binary_path = if let Ok(path) = kernel_test_binary_path.canonicalize() {
		path
	} else {
		log::error!(
			"Path to kernel ('{}') seems to be wrong or file does not exist.",
			kernel_test_binary_path.display()
		);
		process::exit(1);
	};

	if !runner_utils::binary_kind(&kernel_test_binary_path).is_test() {
		log::error!("Kernel test binary does not seem to be a test!?");
		process::exit(1);
	}

	let root_directory = ROOT_DIRECTORY.map_or_else(
		|| {
			if let Ok(handle) = process::Command::new("pwd").output() {
				if let Ok(path) = String::from_utf8(handle.stdout) {
					path
				} else {
					log::error!("Could not parse `pwd` output");
					process::exit(1);
				}
			} else {
				log::error!("Could not run `pwd` and ROOT_DIRECTORY was not given");
				process::exit(1);
			}
		},
		String::from,
	);

	if process::Command::new("cp")
		.current_dir(root_directory.clone())
		.arg(kernel_test_binary_path_string)
		.arg("kernel/build/tests/kernel.bin")
		.status()
		.is_err()
	{
		log::error!("Could not copy binary to test location");
		process::exit(1);
	}

	let shell_script = format!("{}/scripts/run_in_qemu.sh", root_directory);
	let mut run_command = process::Command::new("bash");
	run_command
		.current_dir(root_directory)
		.arg(shell_script)
		.env("QEMU_DIRECTORY", "build/tests");

	match runner_utils::run_with_timeout(&mut run_command, time::Duration::from_secs(TIMEOUT)) {
		Ok(exit_code) => {
			match exit_code.code() {
				// we specifically configured QEMU to
				// exit with exit code 33 on success
				Some(0x3) => {},
				Some(other_exit_code) => {
					log::error!(
						"Tests failed. Exit code was {}.",
						other_exit_code
					);

					process::exit(other_exit_code | 1)
				},
				None => {
					log::error!("Tests failed - terminated by signal?!");
					process::exit(1)
				},
			}
		},
		Err(run_error) => {
			match run_error {
				runner_utils::RunError::TimedOut => {
					log::error!("Test timed out");
					process::exit(1);
				},
				runner_utils::RunError::Io { context, error } => {
					log::error!(
						"I/O error occurred (context = {:?} | error = {:?}",
						context,
						error
					);
					process::exit(1);
				},
			};
		},
	};
}
