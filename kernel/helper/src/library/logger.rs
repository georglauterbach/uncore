// SPDX-License-Identifier: GPL-3.0-or-later

/// ## The Global Test Runner Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide. Shamelessly copied from the kernel code.
pub static LOGGER: Logger = Logger;

/// ## The Kernel Log Level from the Environment
///
/// This variable has a value if the kernel was executed in an environment where
/// the `LOG_LEVEL` environment variable was set.
const LOG_LEVEL: Option<&str> = option_env!("LOG_LEVEL");

/// ### The Main Test Runner Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Logger;

impl Logger
{
	/// ### Parse [`LOG_LEVEL`]
	///
	/// Check if the environment variable `LOG_LEVEL` is set and try to parse
	/// it. Returns [`log::Level::Info`] as the default (if the environment
	/// variable is not present or when the `LOG_LEVEL` variables contains
	/// invalid contents).
	fn from_str() -> log::Level
	{
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

	/// ### Convert [`log::Level`] to [`String`]
	///
	/// Takes a [`log::Level`] variant and converts it accordingly to a [`String`].
	/// This is basically the inverse of [`Logger::from_str`].
	#[must_use]
	pub fn level_to_string(level: &log::Level) -> String
	{
		match level {
			log::Level::Error => "err",
			log::Level::Warn => "war",
			log::Level::Info => "inf",
			log::Level::Debug => "deb",
			log::Level::Trace => "tra",
		}
		.to_string()
	}

	/// ### Set the Log Level
	///
	/// This function takes care of setting the correct log level. If [`None`]
	/// is provided, the "fallback" implementation [`Logger::from_str`] is
	/// used.
	fn set_log_level(log_level: Option<log::Level>)
	{
		if let Some(log_level) = log_level {
			log::set_max_level(log_level.to_level_filter());
			return;
		}

		log::set_max_level(Self::from_str().to_level_filter());
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
/// bootloader information. The default log level chosen if [`None`] is provided
/// is "Info".
pub fn initialize(log_level: Option<log::Level>)
{
	Logger::set_log_level(log_level);
	log::set_logger(&LOGGER).expect("Log should not have already been set");
}
