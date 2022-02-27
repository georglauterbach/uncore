// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ### Link Kernel with Bootloader to Create Bootable Media
///
/// This function links the kernel binary with the `x86_64` bootloader after the kernel
/// was compiled. The bootloader crate can be found under
/// <https://github.com/rust-osdev/bootloader>. It uses a custom linker script for ELF layout, etc.
pub fn link_with_bootloader(test: Option<String>)
{
	use super::environment;
	use ::std::{
		path,
		process,
	};

	let root_directory = environment::get_root_directory().1;
	let root_directory = path::Path::new(&root_directory);

	// path to the bootloader code somewhere on disk
	let bootloader_manifest = bootloader_locator::locate_bootloader("bootloader")
		.expect("Could not locate bootloader manifest");

	// path to the actual kernel binary
	let kernel_binary = test
		.clone()
		.map_or_else(|| environment::get_kernel_binary().1, |test| test);

	let kernel_binary = path::Path::new(&kernel_binary)
		.canonicalize()
		.expect("Could not canonicalize path to kernel binary");

	// Cargo.toml directory of the boot workspace member
	let boot_package_manifest_directory = path::Path::new(env!("CARGO_MANIFEST_DIR"));

	// we know that the kernel lives in the parent directory
	let kernel_directory = boot_package_manifest_directory
		.parent()
		.expect("Could not locate kernel directory");

	// Cargo.toml of the kernel itself
	let kernel_manifest = kernel_directory.join("Cargo.toml");

	// use the same target folder for building the bootloader
	let boot_build_target_directory = kernel_directory.join("target");

	// place the resulting disk image next to our kernel binary
	let disk_image_output_directory = if test.is_some() {
		root_directory.join("kernel/out/tests/boot_output")
	} else {
		root_directory.join("kernel/out/qemu/boot_output")
	};

	// create a new build command; use the `CARGO` environment variable to also support
	// non-standard cargo versions
	let mut build_command = process::Command::new(env!("CARGO"));

	build_command.arg("builder");
	build_command.arg("--firmware").arg("uefi");
	build_command.arg("--kernel-manifest").arg(&kernel_manifest);
	build_command.arg("--kernel-binary").arg(&kernel_binary);
	build_command
		.arg("--target-dir")
		.arg(&boot_build_target_directory);
	build_command.arg("--out-dir").arg(&disk_image_output_directory);

	let bootloader_directory = bootloader_manifest
		.parent()
		.expect("Could not locate bootloader code directory");
	build_command.current_dir(&bootloader_directory);

	let exit_status = build_command
		.status()
		.expect("Bootloader build command did not produce a proper exit status");
	if !exit_status.success() {
		eprintln!("Bootloader build failed");
		process::exit(1);
	}
}
