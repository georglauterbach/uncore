// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use std::{
	env,
	process,
};

/// ### Run `unCORE`
///
/// Runs the kernel image in QEMU.
pub fn run(graphical: bool)
{
	log::debug!("Running unCORE in QEMU now");

	let qemu_directory = if let Ok(directory) = env::var("QEMU_DIRECTORY") {
		directory
	} else {
		"out/qemu".to_string()
	};
	let qemu_volume_directory = qemu_directory.clone() + "/kernel";

	let mut qemu_arguments_one: Vec<&'static str> = vec![
		// "-nodefaults",
		"-machine",
		"q35,accel=kvm:tcg",
		"-m",
		"128M",
		// "-bios",
		// "/usr/share/ovmf/OVMF.fd",
		"-drive",
		"if=pflash,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd,readonly=on",
		"-drive",
		"if=pflash,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd,readonly=on",
		"-device",
		"isa-debug-exit,iobase=0xf4,iosize=0x04",
		"-no-reboot",
	];

	if graphical {
		qemu_arguments_one.push("-vga");
		qemu_arguments_one.push("std");
		qemu_arguments_one.push("-monitor");
		qemu_arguments_one.push("vc:1024x768");
	} else {
		// qemu_arguments_one.push("-nographic");
		qemu_arguments_one.push("-serial");
		qemu_arguments_one.push("stdio");
		qemu_arguments_one.push("-display");
		qemu_arguments_one.push("none");
	}

	let qemu_arguments_two = vec![
		"-drive".to_string(),
		format!("format=raw,file=fat:rw:{}", qemu_volume_directory),
		"-debugcon".to_string(),
		format!("file:{}/debugcon.txt", qemu_directory),
	];

	let exit_code = process::Command::new("qemu-system-x86_64")
		.args(qemu_arguments_one)
		.args(qemu_arguments_two)
		.status()
		.expect("MORJEN")
		.code()
		.unwrap();

	if exit_code == 3 {
		log::info!("Kernel exited QEMU properly");
	} else if exit_code == 0 {
		log::warn!("Kernel exited QEMU unexpectedly (triple-fault, manual QEMU termination, ... ?)");
		process::exit(1);
	} else {
		log::error!("Kernel did not exit QEMU properly (exit code was {})", exit_code);
		process::exit(exit_code + 1);
	}
}
