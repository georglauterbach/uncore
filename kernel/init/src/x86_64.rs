use std::{
	path,
	process,
};

use crate::helper::print_abort_message_and_exit;

/// ### Compile and Run on `x86_64`
///
/// This function compiles the kernel, creates the bootable image and
/// it can additionally run the kernel in QEMU - all for the `x86_64`
/// architecture.
pub(crate) fn main<I>(mut arguments: I)
where
	I: Iterator<Item = String>,
{
	let kernel_path = if let Some(path) = arguments.next() {
		path::PathBuf::from(path)
	} else {
		eprintln!("[ERROR] No path to the kernel binary provided.");
		std::process::exit(1);
	};

	let kernel_path = if let Ok(path) = kernel_path.canonicalize() {
		path
	} else {
		eprintln!(
			"[ERROR] Path to kernel ('{}') seems to be wrong or file does not exist.",
			kernel_path.display()
		);
		std::process::exit(1);
	};

	let mut no_run = false;

	for argument in arguments {
		match argument.as_str() {
			"--no-run" => no_run = true,
			"--do-no-more" => break,
			_ => {
				eprintln!("[ERROR] Argument '{}' is unknown.", argument);
				std::process::exit(1);
			},
		}
	}

	// create the bootable image
	let bios_image = create_disk_images(&kernel_path);

	if runner_utils::binary_kind(&kernel_path).is_test() {
		run_tests(&bios_image);
	} else if no_run {
		println!();
	} else {
		run_in_qemu(&bios_image);
	}
}

/// ### Create the Bootable Image
///
/// Actually runs the boot-image creation process. Returns the path to
/// the bootable image.
#[must_use]
pub fn create_disk_images(kernel_binary_path: &path::Path) -> path::PathBuf
{
	print!("INFO    | Creating disk image...  ");

	let bootloader_manifest_path =
		if let Ok(bootloader_path) = bootloader_locator::locate_bootloader("bootloader") {
			bootloader_path
		} else {
			print_abort_message_and_exit("Could not locate bootloader.");
		};

	let kernel_manifest_path =
		if let Ok(cargo_manifest) = locate_cargo_manifest::locate_manifest() {
			cargo_manifest
		} else {
			print_abort_message_and_exit("Could not locate Cargo manifest.");
		};

	let mut build_command = process::Command::new(env!("CARGO"));

	let bootloader_manifest_path_parent = if let Some(path) = bootloader_manifest_path.parent()
	{
		path
	} else {
		print_abort_message_and_exit("Could not located Cargo.toml parent directory.");
	};

	let kernel_manifest_path_parent = if let Some(path) = kernel_manifest_path.parent() {
		path.join("target")
	} else {
		print_abort_message_and_exit(
			"Could not get the parent directory of the kernel manifest.",
		);
	};

	let kernel_binary_path_parent = if let Some(path) = kernel_binary_path.parent() {
		path
	} else {
		print_abort_message_and_exit(
			"Could not get the parent directory of the kernel binary.",
		);
	};

	build_command.current_dir(bootloader_manifest_path_parent);
	build_command
		.arg("builder")
		.arg("--kernel-manifest")
		.arg(&kernel_manifest_path)
		.arg("--firmware")
		.arg("bios")
		.arg("--kernel-binary")
		.arg(&kernel_binary_path)
		.arg("--target-dir")
		.arg(kernel_manifest_path_parent)
		.arg("--out-dir")
		.arg(kernel_binary_path_parent)
		.arg("--quiet");

	let exit_status = if let Ok(status) = build_command.status() {
		status
	} else {
		print_abort_message_and_exit("Could not run the image builder.");
	};

	if !exit_status.success() {
		print_abort_message_and_exit(format!(
			"Running the image creation process resulted in non-zero exit status '{}'",
			exit_status
		));
	}

	let kernel_binary_name = if let Some(name) = kernel_binary_path.file_name() {
		if let Some(name_as_string) = name.to_str() {
			name_as_string
		} else {
			print_abort_message_and_exit(
				"Could not parse the kernel binary path into a string.",
			);
		}
	} else {
		print_abort_message_and_exit("Could not get the kernel binary's file name");
	};

	let disk_image = if let Some(path) = kernel_binary_path.parent() {
		path.join(format!("boot-bios-{}.img", kernel_binary_name))
	} else {
		print_abort_message_and_exit(
			"Could not locate the kernel binary's parent directory",
		);
	};

	if !disk_image.exists() {
		print_abort_message_and_exit(format!(
			"Disk image does not exist under '{}'.",
			disk_image.display()
		));
	}

	let mut disk_image_directory_path = String::new();
	for (counter, object) in disk_image.iter().rev().enumerate() {
		if counter >= 5 {
			break;
		}

		let object = object.to_str().map_or("", |object| object);
		disk_image_directory_path = format!("{}/{}", object, disk_image_directory_path);
	}

	println!(
		"[ok]\nINFO    | Created disk image at   {}",
		disk_image_directory_path.trim_end_matches('/')
	);

	disk_image
}

/// ### Run the Kernel in QEMU
///
/// This function runs the kernel in QEMU with all parameters set
/// correctly.
fn run_in_qemu(bios_image: &path::Path)
{
	/// arguments given to QEMU when running the kernel
	const QEMU_ARGUMENTS: &[&str] =
		&["--no-reboot", "-s", "-serial", "stdio", "-display", "none"];

	println!("INFO    | Running the kernel in QEMU now.\n");

	let mut run_command = process::Command::new("qemu-system-x86_64");
	run_command
		.arg("-drive")
		.arg(format!("format=raw,file={}", bios_image.display()))
		.args(QEMU_ARGUMENTS);

	let exit_status = run_command.status().unwrap();
	if !exit_status.success() {
		eprintln!(
			"\nERROR   | Running the kernel resulted in non-zero exit status ({})",
			exit_status
		);
		std::process::exit(1);
	}
}

/// ### Running Kernel Tests With QEMU
///
/// This function runs the kernel tests inside QEMU.
fn run_tests(bios_image: &path::Path)
{
	use std::time::Duration;

	/// test timeout duration in seconds
	const TIMEOUT: u64 = 10;
	/// arguments given to QEMU when running the kernel
	const QEMU_ARGUMENTS: &[&str] = &[
		"-device",
		"isa-debug-exit,iobase=0xf4,iosize=0x04",
		"--no-reboot",
		"-s",
		"-serial",
		"stdio",
		"-display",
		"none",
	];

	println!("INFO    | Testing the kernel in QEMU now.\n");

	let mut run_command = process::Command::new("qemu-system-x86_64");
	run_command
		.arg("-drive")
		.arg(format!("format=raw,file={}", bios_image.display()))
		.args(QEMU_ARGUMENTS);

	match runner_utils::run_with_timeout(&mut run_command, Duration::from_secs(TIMEOUT)) {
		Ok(exit_code) => {
			match exit_code.code() {
				// we specifically configured QEMU to
				// exit with exit code 33 on success
				Some(33) => {},
				Some(other_exit_code) => {
					eprintln!(
						"\nERROR   | Tests failed. Exit code was {}.",
						other_exit_code
					);

					std::process::exit(-1)
				},
				None => {
					eprintln!("\nERROR   | Tests failed. Exit code unknown.");
					std::process::exit(-42)
				},
			}
		},
		Err(runner_utils::RunError::TimedOut) => {
			eprintln!("\nERROR   | Test timed out.");
			std::process::exit(-42)
		},
		Err(runner_utils::RunError::Io { context, error }) => {
			eprintln!(
				"\nERROR   | I/O error occurred. (context = {:?}, error = {:?}",
				context, error
			);
			std::process::exit(-42)
		},
	}
}
