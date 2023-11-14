// SPDX-License-Identifier: GPL-3.0-or-later

//! This module provides logging functionality.

/// This static variable is used by the [`log`] crate for
/// logging kernel-wide.
static LOGGER: Logger = Logger;

/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[derive(Debug)]
pub struct Logger;

impl Logger {
  /// ### Set the Log Level
  ///
  /// This function takes care of setting the correct log level. If [`None`]
  /// is provided, the default [`log::Level::Info`] is used.
  fn set_log_level<'a>(log_level: Option<log::Level>) -> &'a str {
    let level_filter = log_level.map_or(log::LevelFilter::Info, |log_level| log_level.to_level_filter());
    log::set_max_level(level_filter);
    level_filter.as_str()
  }
}

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

  fn log(&self, record: &log::Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    /// Shortens the log sequence (writing via `println!`).
    macro_rules! log_with_color {
      ($r:expr, $g:expr, $b:expr) => {{
        use colored::*;
        println!(
          "{:<5} {}",
          record.level().as_str().truecolor($r, $g, $b),
          record.args().to_string()
        )
      }};
    }

    // https://coolors.co/fb4934-fabd2f-458588-83a598-8f8f8f
    match record.level() {
      log::Level::Error => log_with_color!(251, 73, 52),
      log::Level::Warn => log_with_color!(250, 189, 47),
      log::Level::Info => log_with_color!(69, 133, 136),
      log::Level::Debug => log_with_color!(131, 165, 152),
      log::Level::Trace => log_with_color!(143, 143, 143),
    };
  }

  fn flush(&self) {}
}

/// ### Show Initial Information
///
/// This function sets the log level and displays version and
/// bootloader information. The default log level chosen if [`None`] is provided
/// is "Info".
pub fn initialize(log_level: Option<log::Level>) {
  let level = Logger::set_log_level(log_level);
  log::set_logger(&LOGGER).expect("Log should not have already been set");
  log::debug!("Initialized log with log level '{}'", level.to_lowercase());
}
