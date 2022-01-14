// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ### The Kernel Target
///
/// The kernel target is a triple consisting of
///
/// 1. The hardware architecture                        x86_64
/// 2. The vendor (manufacturer) (optional)             unknown
/// 3. Operating system                                 none
/// 4. ABI (optional, omitted in our case)
///
/// The target triple reads as `ARCH-VENDOR-SYS-ABI` and you can read
/// about it [here](https://docs.rust-embedded.org/embedonomicon/custom-target.html).
///
/// The default case for `unCORE` is `x86_64-unknown-none`. This
/// target is for freestanding / bare-metal `x86-64` binaries in ELF
/// format, i.e. firmware, kernels, etc.
const BUILD_TARGET: Option<&str> = option_env!("BUILD_TARGET");

/// ### Compilation Date and Time
///
/// Contains the output of `date +'%H:%M, %d %b %Y'` right before the
/// kernel was compiled.
const COMPILATION_DATE_AND_TIME: Option<&str> = option_env!("COMPILATION_DATE_AND_TIME");

/// ### Rust Toolchain
///
/// Holds the toolchain information that this version of the kernel
/// (stored in `KERNEL_VERSION`) was compiled with.
const RUST_TOOLCHAIN: Option<&str> = option_env!("RUST_TOOLCHAIN");

/// ### Compiler Version
///
/// This variable holds the compiler version that this version of the
/// kernel (stored in `KERNEL_VERSION`) was compiled with.
const RUSTC_VERSION: Option<&str> = option_env!("RUSTC_VERSION");

/// ### Kernel Version
///
/// The `KERNEL_VERSION` variable contains the kernel version in the
/// semantic versioning format, the git commit id the kernel was built
/// with and the build date. If `KERNEL_VERSION` was not available
/// during build-time, a default value is provided, namely "testing".
const KERNEL_VERSION: Option<&str> = option_env!("KERNEL_VERSION");

/// ### Static Kernel Information
///
/// This struct exists to call non-member ("static") function on it to
/// obtain information about the kernel, such as its version or build
/// target as a string.
pub struct KernelInformation;

impl KernelInformation
{
	/// ### Kernel Build Target
	///
	/// Returns the (LLVM) target triple if provided at
	/// built-time. This function tries to make a best-guess by
	/// using conditional compilation if the environment variable
	/// `BUILD_TARGET` was not specified. If nothing was found,
	/// "unknown" is returned.
	#[must_use]
	pub fn get_build_target() -> &'static str
	{
		let target_triple = "unknown";

		#[cfg(all(target_arch = "x86_64", target_abi = "none"))]
		let target_triple = "x86_64-unknown-none";

		BUILD_TARGET.unwrap_or(target_triple)
	}

	/// ### Kernel Compilation Date and Time
	///
	/// Returns the kernel's build date and time, if the
	/// corresponding environment variable was present, otherwise
	/// returns "unknown".
	#[must_use]
	pub fn get_compilation_date_and_time() -> &'static str
	{
		COMPILATION_DATE_AND_TIME.unwrap_or("unknown")
	}

	/// ### Kernel Version
	///
	/// Returns the kernel version if provided at built-time,
	/// otherwise returns "testing".
	#[must_use]
	pub fn get_kernel_version() -> &'static str { KERNEL_VERSION.unwrap_or("testing") }

	/// ### Kernel Rust Toolchain Information
	///
	/// Returns the toolchain information that this version of the
	/// kernel was compiled with if the environment variable was
	/// provided at built-time, otherwise returns "unknown".
	#[must_use]
	pub fn get_rust_toolchain() -> &'static str { RUST_TOOLCHAIN.unwrap_or("unknown") }

	/// ### Kernel Compiler Version
	///
	/// Returns the version of `rustc` that this version of the
	/// kernel was compiled with if the environment variable was
	/// provided at built-time, otherwise returns "unknown".
	#[must_use]
	pub fn get_rustc_version() -> &'static str { RUSTC_VERSION.unwrap_or("unknown") }
}
