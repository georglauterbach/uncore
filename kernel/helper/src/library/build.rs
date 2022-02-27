// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ### Set the Build Target
///
/// This function takes care of setting the build target by setting corresponding
/// variables `BUILD_TARGET_PATH` and `KERNEL_BINARY`.
pub fn set_target(target: &String)
{
	use super::environment;

	/// An array of allowed target names.
	const VALID_TARGETS: &[&str] = &["aarch64", "i686", "x86_64"];

	if !VALID_TARGETS.contains(&target.as_str()) {
		log::error!("Build target '{}' is invalid", target);
		std::process::exit(1);
	}

	let build_target: String = format!("{}{}", target, "-unknown-uncore");

	let build_target_path = format!(
		"{}/kernel/.cargo/targets/{}.json",
		environment::get_root_directory().1,
		build_target
	);
	environment::set_build_target_path(build_target_path);

	let kernel_binary = format!(
		"{}/kernel/target/{}/debug/kernel",
		environment::get_root_directory().1,
		build_target
	);
	environment::set_kernel_binary(kernel_binary);
}
