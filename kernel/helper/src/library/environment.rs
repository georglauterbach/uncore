// SPDX-License-Identifier: GPL-3.0-or-later

/// ### The Path to the Target File
///
/// This variable contains the path to the `.json` file located under
/// `kernel/scripts/.cargo/targets/`. This file contains the information on which target
/// to use and which parameters apply to this target.
static mut BUILD_TARGET_PATH: Option<String> = None;

/// ### Getter for [`BUILD_TARGET_PATH`]
///
/// Returns the string literal name of [`BUILD_TARGET_PATH`] and its value.
#[must_use]
pub fn get_build_target_path() -> (&'static str, String)
{
	("BUILD_TARGET_PATH", unsafe {
		BUILD_TARGET_PATH
			.clone()
			.expect("Expected environment variable 'BUILD_TARGET_PATH' to be set")
	})
}
/// ### Setter for [`BUILD_TARGET_PATH`]
///
/// Sets the value of [`BUILD_TARGET_PATH`].
pub fn set_build_target_path(build_target_path: String)
{
	unsafe {
		BUILD_TARGET_PATH = Some(build_target_path);
	}
}

/// ### The Build Output
///
/// Contains the path of the kernel binary or test binary when `cargo` and `rustc` have
/// compiled the binary.
static mut KERNEL_BINARY: Option<String> = None;

/// ### Getter for [`KERNEL_BINARY`]
///
/// Returns the string literal name of [`KERNEL_BINARY`] and its value.
#[must_use]
pub fn get_kernel_binary() -> (&'static str, String)
{
	("KERNEL_BINARY", unsafe {
		KERNEL_BINARY
			.clone()
			.expect("Expected environment variable 'KERNEL_BINARY' to be set")
	})
}

/// ### Setter for [`KERNEL_BINARY`]
///
/// Sets the value of [`KERNEL_BINARY`].
pub fn set_kernel_binary(kernel_binary: String)
{
	unsafe {
		KERNEL_BINARY = Some(kernel_binary);
	}
}

/// ### The Log Level
///
/// Contains the string representation of the log level.
static mut LOG_LEVEL: Option<String> = None;

/// ### Getter for [`LOG_LEVEL`]
///
/// Returns the string literal name of [`LOG_LEVEL`] and its value.
#[must_use]
pub fn get_log_level() -> (&'static str, String)
{
	("LOG_LEVEL", unsafe {
		LOG_LEVEL
			.clone()
			.expect("Expected environment variable 'LOG_LEVEL' to be set")
	})
}

/// ### Setter for [`LOG_LEVEL`]
///
/// Sets the value of [`LOG_LEVEL`].
pub fn set_log_level(log_level: String)
{
	unsafe {
		LOG_LEVEL = Some(log_level);
	}
}

/// ### When `uncORE` Was Compiled
///
/// Contains date and time information on when the compilation procedure was started.
const COMPILATION_DATE_AND_TIME: Option<&str> = option_env!("COMPILATION_DATE_AND_TIME");

/// ### Getter for [`COMPILATION_DATE_AND_TIME`]
///
/// Returns the string literal name of [`COMPILATION_DATE_AND_TIME`] and its value.
#[must_use]
pub fn get_compilation_date_and_time() -> (&'static str, String)
{
	(
		"COMPILATION_DATE_AND_TIME",
		COMPILATION_DATE_AND_TIME
			.expect("Expected environment variable 'COMPILATION_DATE_AND_TIME' to be set")
			.to_string(),
	)
}

/// ### `unCORE`'s Version
///
/// Contains the `SemVer` version information of this specific `unCORE` kernel binary.
const KERNEL_VERSION: Option<&str> = option_env!("KERNEL_VERSION");

/// ### Getter for [`KERNEL_VERSION`]
///
/// Returns the string literal name of [`KERNEL_VERSION`] and its value.
#[must_use]
pub fn get_kernel_version() -> (&'static str, String)
{
	(
		"KERNEL_VERSION",
		KERNEL_VERSION
			.expect("Expected environment variable 'KERNEL_VERSION' to be set")
			.to_string(),
	)
}

/// ### An Absolute Path
///
/// Contains an absolute path to the repository root. This makes works with the paths
/// easier.
const ROOT_DIRECTORY: Option<&str> = option_env!("ROOT_DIRECTORY");

/// ### Getter for [`ROOT_DIRECTORY`]
///
/// Returns the string literal name of [`ROOT_DIRECTORY`] and its value.
#[must_use]
pub fn get_root_directory() -> (&'static str, String)
{
	(
		"ROOT_DIRECTORY",
		ROOT_DIRECTORY
			.expect("Expected environment variable 'ROOT_DIRECTORY' to be set")
			.to_string(),
	)
}

/// ### Host Toolchain
///
/// Contains information about the host's `rustc` default toolchain and target.
const RUST_DEFAULT_TARGET: Option<&str> = option_env!("RUST_DEFAULT_TARGET");

/// ### Getter for [`RUST_DEFAULT_TARGET`]
///
/// Returns the string literal name of [`RUST_DEFAULT_TARGET`] and its value.
#[must_use]
pub fn get_rust_default_target() -> (&'static str, String)
{
	(
		"RUST_DEFAULT_TARGET",
		RUST_DEFAULT_TARGET
			.expect("Expected environment variable 'RUST_DEFAULT_TARGET' to be set")
			.to_string(),
	)
}

/// ### Compiler Version
///
/// Contains the compiler version `unCORE` was compiled with.
const RUSTC_VERSION: Option<&str> = option_env!("RUSTC_VERSION");

/// ### Getter for [`RUSTC_VERSION`]
///
/// Returns the string literal name of [`RUSTC_VERSION`] and its value.
#[must_use]
pub fn get_rustc_version() -> (&'static str, String)
{
	(
		"RUSTC_VERSION",
		RUSTC_VERSION
			.expect("Expected environment variable 'RUSTC_VERSION' to be set")
			.to_string(),
	)
}

/// ### Kernel Compiler Build Flags
///
/// These flags are needed by `rustc` when building `unCORE`. They take care of setup the
/// compiler built-ins and other needed crates like [`alloc`].
pub const KERNEL_BUILD_FLAGS: &[&str] = &[
	"-Z",
	"build-std=core,compiler_builtins,alloc",
	"-Z",
	"build-std-features=compiler-builtins-mem",
];

/// ### Crate Environment Setup for the Kernel
///
/// This function returns an iterable that is used by [`std::process::Command`]
/// with its `.envs()` method. The vector contains all the environment variables needed
/// when compiling `uncORE`.
#[must_use]
pub fn get_all_environment_variables() -> std::collections::HashMap<&'static str, String>
{
	let mut environment = std::collections::HashMap::new();
	environment.insert(get_build_target_path().0, get_build_target_path().1);
	environment.insert(get_kernel_binary().0, get_kernel_binary().1);
	environment.insert(get_log_level().0, get_log_level().1);
	environment.insert(
		get_compilation_date_and_time().0,
		get_compilation_date_and_time().1,
	);
	environment.insert(get_kernel_version().0, get_kernel_version().1);
	environment.insert(get_root_directory().0, get_root_directory().1);
	environment.insert(get_rust_default_target().0, get_rust_default_target().1);
	environment.insert(get_rustc_version().0, get_rustc_version().1);

	environment
}
