// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use helper::environment;
use std::process;

/// ### Run Checks
///
/// Runs all code against `clippy`, `rustfmt` and `rustdoc` to check it.
pub fn check(is_ci: bool)
{
	log::info!("Running 'cargo check'");
	let kernel_check = process::Command::new(env!("CARGO"))
		.args(&["check", "--target"])
		.arg(environment::get_build_target_path().1)
		.args(environment::KERNEL_BUILD_FLAGS)
		.envs(environment::get_all_environment_variables())
		.status();

	log::info!("Checking the source code documentation");
	let documentation_check = process::Command::new(env!("CARGO"))
		.args(&["doc", "--lib", "--document-private-items"])
		.status();

	log::info!("Running formatting and clippy checks");
	let format_check = process::Command::new(env!("CARGO"))
		.args(&["fmt", "--all", "--message-format", "human", "--", "--check"])
		.status();
	let clippy_lib = process::Command::new(env!("CARGO"))
		.args(&["clippy", "--lib", "--all-features", "--", "-D", "warnings"])
		.status();
	let clippy_test_runner = process::Command::new(env!("CARGO"))
		.args(&[
			"clippy",
			"--package",
			"test_runner",
			"--all-features",
			"--",
			"-D",
			"warnings",
		])
		.status();
	let clippy_helper = process::Command::new(env!("CARGO"))
		.args(&[
			"clippy",
			"--package",
			"helper",
			"--all-features",
			"--",
			"-D",
			"warnings",
		])
		.status();

	check_result("default kernel", &kernel_check, is_ci);
	check_result("documentation", &documentation_check, is_ci);
	check_result("formatting", &format_check, is_ci);
	check_result("clippy library", &clippy_lib, is_ci);
	check_result("clippy test runner", &clippy_test_runner, is_ci);
	check_result("clippy helper", &clippy_helper, is_ci);
}

/// ### Check a Given Test Result
///
/// Shows information about the test outcome. Exits early with exit code 1 if `is_ci` is
/// `true` and a tests failed.
fn check_result(name: &'static str, result: &Result<process::ExitStatus, std::io::Error>, is_ci: bool)
{
	if result.is_ok_with(process::ExitStatus::success) {
		log::info!("Check '{}' succeeded", name);
	} else if is_ci {
		log::error!("Check '{}' failed", name);
		process::exit(1);
	} else {
		log::warn!("Check '{}' failed", name);
	}
}

/// ### Run Tests
///
/// This function runs tests by using the `test_runner` workspace member. Of `test` is
/// supplied, only unit tests or a single integration test is run.
pub fn test(test: Option<String>, is_ci: bool)
{
	let mut command = process::Command::new(env!("CARGO"));
	let command = command
		.args(&["test", "--target"])
		.arg(environment::get_build_target_path().1)
		.args(environment::KERNEL_BUILD_FLAGS)
		.envs(environment::get_all_environment_variables());

	let command = if let Some(test) = test {
		if test == "lib" {
			log::info!("Running unit tests of lib.rs");
			command.arg("--lib")
		} else {
			log::info!("Running integration test '{}'", test);
			command.arg("--test").arg(test)
		}
	} else {
		log::info!("Running all tests");
		command.arg("--tests")
	};

	let status = command.status().expect("namd");

	if status.success() {
		log::info!("Test(s) passed");
	} else if is_ci {
		log::error!("Test(s) failed ({:?})", status);
		process::exit(1);
	} else {
		log::warn!("Test(s) failed ({:?})", status);
	}
}
