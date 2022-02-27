// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use helper::{
	bootloader,
	environment,
};
use std::{
	env,
	process,
};

/// ### Build `unCORE`
///
/// Builds the kernel image and links it with the bootloader.
pub fn build()
{
	log::debug!("Starting kernel build process");

	if !process::Command::new(env!("CARGO"))
		.arg("build")
		.arg("--target")
		.arg(environment::get_build_target_path().1)
		.args(environment::KERNEL_BUILD_FLAGS)
		.envs(environment::get_all_environment_variables())
		.status()
		.expect("Kernel build command did not produce a proper exit status")
		.success()
	{
		log::error!("Could not compile kernel");
		process::exit(1);
	};

	log::debug!("Finished building the kernel");
	log::debug!("Linking kernel with bootloader now");

	bootloader::link(&None);

	let bootloader_build_output = format!(
		"{}/kernel/out/qemu/boot_output/boot-uefi-kernel.efi",
		environment::get_root_directory().1
	);

	if !process::Command::new("cp")
		.arg(&bootloader_build_output)
		.arg(format!(
			"{}/kernel/out/qemu/kernel/EFI/BOOT/BOOTX64.EFI",
			environment::get_root_directory().1
		))
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
