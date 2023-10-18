// SPDX-License-Identifier: GPL-3.0-or-later

//! This module provides logging functionality.

/// ## The Global Test Runner Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide. Shamelessly copied from the kernel code.
static LOGGER: Logger = Logger;

/// ### The Main Test Runner Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Logger;

impl Logger {
  /// ### Set the Log Level
  ///
  /// This function takes care of setting the correct log level. If [`None`]
  /// is provided, the "fallback" implementation [`Logger::from_str`] is
  /// used.
  fn set_log_level(log_level: Option<log::Level>) {
    let level_filter = log_level.map_or(log::LevelFilter::Info, |log_level| log_level.to_level_filter());
    log::set_max_level(level_filter);
  }
}

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

  fn log(&self, record: &log::Record) {
    use ansi_rgb::Foreground;
    use log::Level;
    use rgb::RGB8;

    if !self.enabled(record.metadata()) {
      return;
    }

    // https://coolors.co/fb4934-fabd2f-458588-83a598-8f8f8f
    let (log_level, color) = match record.level() {
      Level::Error => ("ERROR", RGB8::new(251, 73, 52)),
      Level::Warn => ("WARN ", RGB8::new(250, 189, 47)),
      Level::Info => ("INFO ", RGB8::new(69, 133, 136)),
      Level::Debug => ("DEBUG", RGB8::new(131, 165, 152)),
      Level::Trace => ("TRACE", RGB8::new(143, 143, 143)),
    };

    println!(
      "{} {:<15.*} | {}",
      log_level.fg(color),
      25,
      record.module_path().unwrap_or("unknown"),
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
pub fn initialize(log_level: Option<log::Level>) {
  Logger::set_log_level(log_level);
  log::set_logger(&LOGGER).expect("Log should not have already been set");
}
