// SPDX-License-Identifier: GPL-3.0-or-later

//! This module is responsible for the kernel log.

/// ## The Global Kernel Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide.
static LOGGER: KernelLogger = KernelLogger::new();

/// ### The Main Kernel Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[derive(Debug)]
pub struct KernelLogger {
  /// This field represents a UART provides by QEMU
  qemu_uart: qemu_uart::Logger,
}

impl KernelLogger {
  /// ### Create a New Global Logger for the Kernel
  ///
  /// Creates a new instance of the kernel-wide logger.
  const fn new() -> Self {
    Self {
      qemu_uart: qemu_uart::Logger,
    }
  }

  /// This function takes care of setting the correct log level.
  fn set_log_level(log_level: log::Level) { log::set_max_level(log_level.to_level_filter()); }

  /// As the QEMU UART logger is disabled in the beginning, we need to enable it
  /// explicitly after the UART has been initialed.
  pub fn enable_qemu_logger() { qemu_uart::Logger::enable(); }
}

impl log::Log for KernelLogger {
  fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

  fn log(&self, record: &log::Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    self.qemu_uart.log(record);
  }

  fn flush(&self) {}
}

/// Initializes the log by setting the global kernel logger and the correct log level.
///
/// #### Panics
///
/// If this function is called twice, the kernel panics, because we want to avoid code
/// that initializes the logger twice.
pub fn initialize() {
  crate::panic_on_error!(log::set_logger, &LOGGER);
  KernelLogger::set_log_level(super::env::KernelInformation::get_log_level());

  log::debug!("Kernel logging enabled");

  log::debug!(
    "Log level set to '{}'",
    super::env::KernelInformation::get_log_level()
  );
}

/// ### Print Initial Information
///
/// This function displays initial information during startup of the kernel, like its
/// version, when it was compiled, how it was compiled, etc.
pub fn display_initial_information() {
  log::info!(
    "Welcome to unCORE version {}",
    super::env::KernelInformation::get_kernel_version()
  );
  log::trace!(
    "unCORE was compiled at {}",
    super::env::KernelInformation::get_compilation_date_and_time()
  );
  log::trace!(
    "unCORE was compiled with {} and toolchain {}",
    super::env::KernelInformation::get_rustc_version(),
    super::env::KernelInformation::get_rust_toolchain()
  );
}

/// This module contains code to work with the UART provided by the underlying
/// architecture, see [`crate::arch::drivers::qemu_uart::Uart`].
mod qemu_uart {
  /// This lock ensure that simultaneous writers will be serialized when calling [`log`].
  /// The logger is initialized in a way that logging has to be enabled explicitly later
  /// (when the UART itself has been initialized), as logging before that is undefined
  /// behavior.
  ///
  /// The first field indicates whether the logger is enabled or not. We have to store
  /// this information in the same [`spin::Mutex`] to avoid race-conditions later when
  /// locking the Mutex and checking whether the logger is enabled.
  ///
  /// We also not introduce a new filed on [`Logger`] because we would need to mutate
  /// through a `&self` reference (to disable the logger is required) in
  /// [`log::Log::log`], which is not allowed.
  static LOCK: spin::Mutex<(bool, crate::arch::drivers::qemu_uart::Uart)> =
    spin::Mutex::new((false, crate::arch::drivers::qemu_uart::UART));

  /// An opaque type used to implement [`log::Log`] on.
  #[derive(Debug)]
  pub struct Logger;

  impl Logger {
    /// Enables this logger.
    pub(super) fn enable() { LOCK.lock().0 = true; }
  }

  impl log::Log for Logger {
    // This function is not used because the global logger instance already checks whether the
    // log is enabled or not.
    fn enabled(&self, _: &log::Metadata) -> bool { true }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
      let mut lock = LOCK.lock();

      if lock.0 {
        use core::fmt::Write;
        use owo_colors::OwoColorize;

        /// Shortens the log sequence (writing via `println!`).
        macro_rules! log_with_color {
          ($string:expr, $r:expr, $g:expr, $b:expr) => {{
            if let Err(_) = writeln!(
              lock.1,
              "{} {}",
              $string.fg_rgb::<$r, $g, $b>(),
              record.args()
            ) {
              lock.0 = false;
            }
          }};
        }

        // https://coolors.co/fb4934-fabd2f-458588-83a598-8f8f8f
        match record.level() {
          log::Level::Error => log_with_color!("ERROR", 251, 73, 52),
          log::Level::Warn => log_with_color!("WARN ", 250, 189, 47),
          log::Level::Info => log_with_color!("INFO ", 69, 133, 136),
          log::Level::Debug => log_with_color!("DEBUG", 131, 165, 152),
          log::Level::Trace => log_with_color!("TRACE", 143, 143, 143),
        };
      }
    }
  }
}
