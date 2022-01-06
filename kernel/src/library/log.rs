// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// TODO implement the `log` (crate) facade

use ::core::fmt;

use crate::prelude::*;

/// ### The Logging Severity Level
///
/// The `LOG_LEVEL` is used to define which messages are being logged.
/// All messaged with an equal or higher priority are logged when not
/// running tests. When running tests, all messages with a severity of
/// `Level::Warning` or higher are logged.
static mut LOG_LEVEL: Level = if test::IS_TEST {
	Level::Trace
} else {
	Level::Info
};

/// ### Log Level
///
/// This struct holds the five well-known log levels `Trace`, `Debug`,
/// `info`, `Warning`, `Error` and `Fatal` as well as the `Test` log
/// level which is for logging during tests.
#[doc(hidden)]
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub enum Level
{
	Trace,
	Debug,
	Info,
	Warning,
	Error,
	Fatal,
	Test,
	None,
}

impl fmt::Display for Level
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match &self {
			Self::Trace => write!(f, "TRACE   | "),
			Self::Debug => write!(f, "DEBUG   | "),
			Self::Info => write!(f, "INFO    | "),
			Self::Warning => write!(f, "WARNING | "),
			Self::Error => write!(f, "ERROR   | "),
			Self::Fatal => write!(f, "FATAL   | "),
			Self::Test => write!(f, "TEST    | "),
			Self::None => write!(f, ""),
		}
	}
}

impl core::default::Default for Level
{
	fn default() -> Self { Self::Trace }
}

/// ### Show Initial Information
///
/// This function sets the log level and displays version and
/// bootloader information.
pub fn init(log_level: Option<Level>)
{
	set_log_level(log_level.unwrap_or_default());

	display_initial_information();
	log_info!("Post-UEFI initialization started");
}

/// ### Set the Kernel Log Level
///
/// This function adjusts the kernel log level. Only call this
/// function once and at the very start if necessary.
pub fn set_log_level(new_log_level: Level)
{
	unsafe {
		LOG_LEVEL = new_log_level;
	}
}

/// ### Log Indirection
///
/// An indirection that is used in order to make it easy to switch to
/// different loggers or log to multiple interfaces. The indirection
/// is nice because now we do not need to make the different output
/// modules (such as `serial`) in this module public.
#[doc(hidden)]
pub const fn __log(_log_level: Level, _arguments: fmt::Arguments)
{
	// if log_level < unsafe { LOG_LEVEL } {
	// 	return;
	// }

	// serial::print(format_args!("{}{}\n", log_level,
	// arguments));
}

/// ### Print Welcome
///
/// This function is just here to log an unmodified line. It exists to
/// print the welcome message of the kernel and should not be used
/// somewhere else. Please use the log macros provided below that
/// designate a certain log level. It is used for tests to show the
/// `[ok]` message after a test has finished successfully.
#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::None,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::TRACE`
///
/// The highest log level here for very, very detailed output.
#[macro_export]
macro_rules! log_trace {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Trace,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::DEBUG`
///
/// For debugging purposes and very detailed output.
#[macro_export]
macro_rules! log_debug {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Debug,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::INFO`
///
/// Log informational output with this macro.
#[macro_export]
macro_rules! log_info {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Info,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::WARNING`
///
/// This log level makes the reader aware something may not be
/// correct.
#[macro_export]
macro_rules! log_warning {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Warning,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::ERROR`
///
/// Show that something definitely is incorrect, but not
/// unrecoverable.
#[macro_export]
macro_rules! log_error {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Error,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::FATAL`
///
/// Show that something went very wrong. This indicates a
/// unrecoverable situation, such as a double fault.
#[macro_export]
macro_rules! log_fatal {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Fatal,
			format_args!($($arg)*)
		)
	};
}

/// ### Log with `Level::TEST`
///
/// This log level shall be used when running tests.
#[macro_export]
macro_rules! log_test {
	($($arg:tt)*) => {
		$crate::library::log::__log(
			$crate::library::log::Level::Test,
			format_args!($($arg)*)
		)
	};
}

/// ### Print Initial Information
///
/// We print out information about the kernel, for example
///
/// - its version
/// - its (LLVM) target triple
/// - the compiler version
/// - Rust toolchain information
pub fn display_initial_information()
{
	log!("This is unCORE {}\n", KernelInformation::get_version());

	log_trace!(
		"Target triple reads '{}'",
		KernelInformation::get_build_target()
	);
	log_trace!(
		"This version of unCORE was compiled with rustc version '{}'",
		KernelInformation::get_rustc_version()
	);
	log_trace!(
		"This version of unCORE was using the toolchain '{}'",
		KernelInformation::get_rust_toolchain()
	);
}
