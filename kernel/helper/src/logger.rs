// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## The Global Test Runner Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide. Shamelessly copied from the kernel code.
pub static LOGGER: Logger = Logger;

/// ## The Kernel Log Level from the Environment
///
/// This variable has a value if the kernel was executed in an environment where the
/// `LOG_LEVEL` environment variable was set.
const LOG_LEVEL: Option<&str> = option_env!("LOG_LEVEL");

/// ### The Main Test Runner Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[allow(clippy::module_name_repetitions)]
pub struct Logger;

impl Logger
{
	/// ### Parse [`LOG_LEVEL`]
	///
	/// Check if the environment variable `LOG_LEVEL` is set and try to parse it.
	/// Returns [`None`] the string could not be parsed or if the environment variable
	/// is not present.
	fn try_from_str() -> Option<log::Level>
	{
		LOG_LEVEL.and_then(|log_level| match log_level {
			"err" => Some(log::Level::Error),
			"war" => Some(log::Level::Warn),
			"inf" => Some(log::Level::Info),
			"deb" => Some(log::Level::Debug),
			"tra" => Some(log::Level::Trace),
			_ => None,
		})
	}

	/// ### Set the Log Level
	///
	/// This function takes care of setting the correct log level.
	fn set_log_level(log_level: log::Level)
	{
		Self::try_from_str().map_or_else(
			|| {
				log::set_max_level(log_level.to_level_filter());
			},
			|log_level| {
				log::set_max_level(log_level.to_level_filter());
			},
		);
	}
}

impl log::Log for Logger
{
	fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

	fn log(&self, record: &log::Record)
	{
		use ansi_rgb::Foreground;
		use log::Level;
		use rgb::RGB8;

		if !self.enabled(record.metadata()) {
			return;
		}

		// https://coolors.co/fb4934-fabd2f-458588-83a598-8f8f8f
		let (log_level, color) = match record.level() {
			Level::Error => (" ERROR ", RGB8::new(251, 73, 52)),
			Level::Warn => ("WARNING", RGB8::new(250, 189, 47)),
			Level::Info => ("  INF  ", RGB8::new(69, 133, 136)),
			Level::Debug => (" DEBUG ", RGB8::new(131, 165, 152)),
			Level::Trace => (" TRACE ", RGB8::new(143, 143, 143)),
		};

		println!(
			"[ {} ] {:>25.*}{}{:<4.*} | {}",
			log_level.fg(color),
			25,
			record.file().unwrap_or("unknown"),
			"@".fg(color),
			4,
			record.line().unwrap_or(0),
			record.args().fg(color)
		);
	}

	fn flush(&self) {}
}

/// ### Show Initial Information
///
/// This function sets the log level and displays version and
/// bootloader information.
pub fn initialize(log_level: log::Level)
{
	Logger::set_log_level(log_level);
	log::set_logger(&LOGGER).expect("Log should not have already been set");
}
