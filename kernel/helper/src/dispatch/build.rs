// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use super::environment;
use std::{
	env,
	process,
};

pub(crate) fn build()
{
	log::debug!("Starting kernel build process");

	if !process::Command::new(env!("CARGO"))
		.arg("build")
		.arg("--target")
		.arg(environment::get_build_target_path().1)
		.args(environment::KERNEL_BUILD_FLAGS)
		.envs(environment::kernel_environment())
		.status()
		.expect("Kernel build command did not produce a proper exit status")
		.success()
	{
		log::error!("Could not compile kernel");
		process::exit(1);
	};

	log::debug!("Finished building the kernel");
	log::debug!("Linking kernel with bootloader now");

	if !process::Command::new(env!("CARGO"))
		.arg("run")
		.arg("--package")
		.arg("boot")
		.arg("--quiet")
		.envs(environment::kernel_environment())
		.status()
		.expect("Kernel build command did not produce a proper exit status")
		.success()
	{
		log::error!("Could not link the kernel with the bootloader");
		process::exit(1);
	}

	let bootloader_build_output = format!(
		"{}/kernel/out/qemu/boot_output/boot-uefi-kernel.efi",
		environment::get_root_directory().1
	);

	if !process::Command::new("cp")
		.arg(&bootloader_build_output)
		.arg(environment::get_qemu_kernel_binary().1)
		.status()
		.expect("Kernel build command did not produce a proper exit status")
		.success()
	{
		log::error!(
			"Could not copy bootloader build output: '{}'",
			bootloader_build_output
		);
		process::exit(1);
	}

	log::debug!("Created bootable kernel image(s)");
}

pub(crate) fn set_target(target: &String)
{
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
