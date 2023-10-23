// SPDX-License-Identifier: GPL-3.0-or-later

/// ### Compilation Date and Time
///
/// Contains the output of `date +'%H:%M, %d %b %Y'` right before the
/// kernel was compiled.
const COMPILATION_DATE_AND_TIME: Option<&str> = option_env!("COMPILATION_DATE_AND_TIME");

/// ### Kernel Version
///
/// The `KERNEL_VERSION` variable contains the kernel version in the
/// semantic versioning format, the git commit id the kernel was built
/// with and the build date. If `KERNEL_VERSION` was not available
/// during build-time, a default value is provided, namely "testing".
const KERNEL_VERSION: Option<&str> = option_env!("KERNEL_VERSION");

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

/// ## The Kernel Log Level from the Environment
///
/// This variable has a value if the kernel was executed in an environment where the
/// `LOG_LEVEL` environment variable was set.
pub(super) const LOG_LEVEL: Option<&str> = option_env!("LOG_LEVEL");

/// ### Static Kernel Information
///
/// This struct exists to call non-member ("static") function on it to
/// obtain information about the kernel, such as its version or build
/// target as a string.
#[derive(Debug, Copy, Clone)]
pub struct KernelInformation;

impl KernelInformation {
  /// ### Kernel Compilation Date and Time
  ///
  /// Returns the kernel's build date and time, if the
  /// corresponding environment variable was present, otherwise
  /// returns "unknown".
  #[must_use]
  pub fn get_compilation_date_and_time() -> &'static str { COMPILATION_DATE_AND_TIME.unwrap_or("unknown") }

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

  pub fn get_log_level() -> log::Level {
    LOG_LEVEL.map_or_else(
      || log::Level::Info,
      |log_level| match log_level {
        "err" => log::Level::Error,
        "war" => log::Level::Warn,
        "deb" => log::Level::Debug,
        "tra" => log::Level::Trace,
        _ => log::Level::Info,
      },
    )
  }
}
