// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ### The Kernel Target
///
/// The kernel target is a triple consisting of
///
/// 1. The hardware architecture
/// 2. The vendor (manufacturer) (optional)
/// 3. Operating system
/// 4. ABI (omitted in our case)
///
/// The target triple reads as `ARCH-VENDOR-SYS-ABI` and you can read
/// about it [here](https://docs.rust-embedded.org/embedonomicon/custom-target.html).
///
/// The default case for `unCORE` is `x86_64-unknown-none`. This
/// target is for freestanding / bare-metal `x86-64` binaries in ELF
/// format, i.e. firmware, kernels, etc.
const BUILD_TARGET: Option<&str> = option_env!("BUILD_TARGET");

/// ### Rust Toolchain
///
/// Holds the toolchain information that this version of the kernel
/// (stored in `VERSION`) was compiled with.
const RUST_TOOLCHAIN: Option<&str> = option_env!("RUST_TOOLCHAIN");

/// ### Compiler Version
///
/// This variable holds the compiler version that this version of the
/// kernel (stored in `VERSION`) was compiled with.
const RUSTC_VERSION: Option<&str> = option_env!("RUSTC_VERSION");

/// ### Kernel Version
///
/// The `VERSION` variable contains the kernel version in the semantic
/// versioning format, the git commit id the kernel was built with and
/// the build date. If `VERSION` was not available during build-time,
/// a default value is provided, namely "testing".
const VERSION: Option<&str> = option_env!("VERSION");

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
		#[cfg(all(target_arch = "x86_64", target_abi = "none"))]
		let target_triple: &str = "x86_64-unknown-none";

		#[cfg(not(all(target_arch = "x86_64", target_abi = "none")))]
		let target_triple: &str = "unknown";

		BUILD_TARGET.unwrap_or(target_triple)
	}

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

	/// ### Kernel Version
	///
	/// Returns the kernel version if provided at built-time,
	/// otherwise returns "testing".
	#[must_use]
	pub fn get_version() -> &'static str { VERSION.unwrap_or("testing") }
}
