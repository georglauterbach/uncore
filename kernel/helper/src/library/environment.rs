// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

static mut BUILD_TARGET_PATH: Option<String> = None;

pub fn get_build_target_path() -> (&'static str, String)
{
	("BUILD_TARGET_PATH", unsafe {
		BUILD_TARGET_PATH
			.clone()
			.expect("Expected environment variable 'BUILD_TARGET_PATH' to be set")
	})
}

pub fn set_build_target_path(build_target_path: String)
{
	unsafe {
		BUILD_TARGET_PATH = Some(build_target_path);
	}
}

static mut KERNEL_BINARY: Option<String> = None;

pub fn get_kernel_binary() -> (&'static str, String)
{
	("KERNEL_BINARY", unsafe {
		KERNEL_BINARY
			.clone()
			.expect("Expected environment variable 'KERNEL_BINARY' to be set")
	})
}

pub fn set_kernel_binary(kernel_binary: String)
{
	unsafe {
		KERNEL_BINARY = Some(kernel_binary);
	}
}

static mut LOG_LEVEL: Option<String> = None;

pub fn get_log_level() -> (&'static str, String)
{
	("LOG_LEVEL", unsafe {
		LOG_LEVEL
			.clone()
			.expect("Expected environment variable 'LOG_LEVEL' to be set")
	})
}

pub fn set_log_level(log_level: String)
{
	unsafe {
		LOG_LEVEL = Some(log_level);
	}
}

const COMPILATION_DATE_AND_TIME: Option<&str> = option_env!("COMPILATION_DATE_AND_TIME");

pub fn get_compilation_date_and_time() -> (&'static str, String)
{
	(
		"COMPILATION_DATE_AND_TIME",
		COMPILATION_DATE_AND_TIME
			.expect("Expected environment variable 'COMPILATION_DATE_AND_TIME' to be set")
			.to_string(),
	)
}

const KERNEL_VERSION: Option<&str> = option_env!("KERNEL_VERSION");

pub fn get_kernel_version() -> (&'static str, String)
{
	(
		"KERNEL_VERSION",
		KERNEL_VERSION
			.expect("Expected environment variable 'KERNEL_VERSION' to be set")
			.to_string(),
	)
}

const ROOT_DIRECTORY: Option<&str> = option_env!("ROOT_DIRECTORY");

pub fn get_root_directory() -> (&'static str, String)
{
	(
		"ROOT_DIRECTORY",
		ROOT_DIRECTORY
			.expect("Expected environment variable 'ROOT_DIRECTORY' to be set")
			.to_string(),
	)
}

const RUST_DEFAULT_TARGET: Option<&str> = option_env!("RUST_DEFAULT_TARGET");

pub fn get_rust_default_target() -> (&'static str, String)
{
	(
		"RUST_DEFAULT_TARGET",
		RUST_DEFAULT_TARGET
			.expect("Expected environment variable 'RUST_DEFAULT_TARGET' to be set")
			.to_string(),
	)
}

const RUSTC_VERSION: Option<&str> = option_env!("RUSTC_VERSION");

pub fn get_rustc_version() -> (&'static str, String)
{
	(
		"RUSTC_VERSION",
		RUSTC_VERSION
			.expect("Expected environment variable 'RUSTC_VERSION' to be set")
			.to_string(),
	)
}

pub const KERNEL_BUILD_FLAGS: &[&str] = &[
	"-Z",
	"build-std=core,compiler_builtins,alloc",
	"-Z",
	"build-std-features=compiler-builtins-mem",
];

pub fn kernel_environment() -> Vec<(&'static str, String)>
{
	vec![
		get_build_target_path(),
		get_kernel_binary(),
		get_log_level(),
		get_compilation_date_and_time(),
		get_kernel_version(),
		get_root_directory(),
		get_rust_default_target(),
		get_rustc_version(),
	]
}
