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
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]

//! # The `unCORE` Test Runner Integration
//!
//! This workspace member enabled running unit- and integration tests
//! seamlessly. This test runner should only be invoked by the script
//! `test_kernel.sh` under `scripts/`.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS / VARIABLES
// ? ---------------------------------------------------------------------

use std::{
	path,
	process,
	time,
};

use helper::{
	bootloader,
	environment,
	logger,
};

/// ### Individual Test Run Timeout
///
/// The time **in seconds** any individual test has before being
/// terminated. This does not apply to the kernel main binary.
const TIMEOUT: u64 = 30;

/// ### Entrypoint
///
/// This is a simple, nice, beautiful `main` function, right?
fn main()
{
	logger::initialize(None);
	log::info!("Started test runner");

	let kernel_test_binary_path_string = std::env::args().skip(1).next().map_or_else(
		|| {
			log::error!("No path to the kernel binary provided.");
			process::exit(1);
		},
		String::from,
	);
	let kernel_test_binary_path = path::PathBuf::from(kernel_test_binary_path_string.clone());

	environment::set_build_target_path(
		option_env!("BUILD_TARGET_PATH")
			.expect("Expected environment variable 'BUILD_TARGET_PATH' to be set")
			.to_string(),
	);
	environment::set_kernel_binary(
		option_env!("KERNEL_BINARY")
			.expect("Expected environment variable 'KERNEL_BINARY' to be set")
			.to_string(),
	);
	environment::set_log_level(
		option_env!("LOG_LEVEL")
			.expect("Expected environment variable 'LOG_LEVEL' to be set")
			.to_string(),
	);

	// miscellaneous checks and adjustments (path canonicalization, etc.)
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

	let kernel_test_binary_name = kernel_test_binary_path.file_name().map_or_else(
		|| {
			log::error!("Could not acquire kernel test binary file name");
			process::exit(1);
		},
		|name| {
			name.to_str()
				.expect("String conversion for kernel test binary name failed")
		},
	);

	// linking tets binary with bootloader here
	bootloader::link_with_bootloader(Some(kernel_test_binary_path_string.to_string()));

	let bootloader_build_output = format!(
		"{}/kernel/out/tests/boot_output/boot-uefi-{}.efi",
		environment::get_root_directory().1,
		kernel_test_binary_name
	);

	if !process::Command::new("cp")
		.arg(bootloader_build_output)
		.arg(format!(
			"{}/kernel/out/tests/kernel/EFI/BOOT/BOOTX64.EFI",
			environment::get_root_directory().1
		))
		.status()
		.expect("Kernel test build command did not produce a proper exit status")
		.success()
	{
		log::error!("Could not copy binary to test location");
		process::exit(1);
	}

	if kernel_test_binary_path_string.contains("uncore/debug/deps/kernel-") {
		run_test(120);
	} else {
		run_test(TIMEOUT);
	}
}

/// ### Run the Actual Tests
///
/// This function runs the test binary in QEMU properly by calling the `run_in_qemu.sh`
/// script with the correct environment and with proper timeout.
fn run_test(timeout: u64)
{
	log::debug!("Test runner runs test now");

	let mut run_command = process::Command::new(env!("CARGO"));
	run_command
		.args(&["run", "--package", "helper", "--", "run"])
		.env("QEMU_DIRECTORY", "out/tests");

	match runner_utils::run_with_timeout(&mut run_command, time::Duration::from_secs(timeout)) {
		Ok(exit_code) => match exit_code.code() {
			Some(0) => {},
			Some(other_exit_code) => {
				log::error!("Tests failed. Exit code was {}.", other_exit_code);

				process::exit(other_exit_code | 1)
			},
			None => {
				log::error!("Tests failed - terminated by signal?!");
				process::exit(1)
			},
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
