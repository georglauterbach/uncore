// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## The Global Test Runner Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide. Shamelessly copied from the kernel code.
pub static LOGGER: KernelTestRunnerLogger = KernelTestRunnerLogger;

/// ### The Main Test Runner Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[allow(clippy::module_name_repetitions)]
pub struct KernelTestRunnerLogger;

impl log::Log for KernelTestRunnerLogger
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

		// https://coolors.co/da3e52-f2e94e-a3d9ff-bdefcf-9fa4a8
		let (log_level, color) = match record.level() {
			Level::Error => (" ERROR ", RGB8::new(218, 62, 82)),
			Level::Warn => ("WARNING", RGB8::new(242, 233, 78)),
			Level::Info => ("  INF  ", RGB8::new(163, 217, 255)),
			Level::Debug => (" DEBUG ", RGB8::new(189, 239, 207)),
			Level::Trace => (" TRACE ", RGB8::new(159, 164, 168)),
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
pub fn init(log_level: log::Level)
{
	log::set_max_level(log_level.to_level_filter());
	log::set_logger(&LOGGER).expect("Log should not have already been set");
}
