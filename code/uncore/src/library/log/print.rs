// SPDX-License-Identifier: GPL-3.0-or-later

/// ## The Global Kernel Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide.
// TODO this can somehow not be `static mut`? (linker script?) -> can we use a Mutex?
static LOGGER: KernelLogger = KernelLogger::new();

/// ### The Main Kernel Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[derive(Debug)]
pub struct KernelLogger {
  /// TODO
  qemu_uart:         qemu_uart::Logger,
  /// TODO
  qemu_uart_enabled: bool,
}

impl KernelLogger {
  /// TODO
  pub fn enable_or_disable_qemu_uart(&mut self, enabled: bool) { self.qemu_uart_enabled = enabled; }

  /// ### Create a New Global Logger for the Kernel
  ///
  /// Creates a new instance of the kernel-wide logger.
  const fn new() -> Self {
    Self {
      qemu_uart:         qemu_uart::Logger::new(),
      qemu_uart_enabled: true,
    }
  }

  /// ### Set the Log Level
  ///
  /// This function takes care of setting the correct log level. If [`None`]
  /// is provided, the "fallback" implementation [`KernelLogger::from_str`] is
  /// used.
  fn set_log_level(log_level: log::Level) { log::set_max_level(log_level.to_level_filter()); }
}

impl log::Log for KernelLogger {
  fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

  fn log(&self, record: &log::Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    if self.qemu_uart_enabled {
      self.qemu_uart.log(record);
    }
  }

  fn flush(&self) {}
}

/// TODO
pub fn initialize() {
  KernelLogger::set_log_level(log::Level::Trace);
  log::set_logger(unsafe { &LOGGER }).expect("Log should not have already been set");
  log::debug!("Kernel logging enabled");
}

/// ### Print Initial Information
///
/// TODO
pub fn display_initial_information() {
  log::error!(
    "Welcome to unCORE version {}",
    super::env::KernelInformation::get_kernel_version()
  );
  log::debug!(
    "unCORE was compiled at {}",
    super::env::KernelInformation::get_compilation_date_and_time()
  );
  log::debug!(
    "unCORE was compiled with {} and toolchain {}",
    super::env::KernelInformation::get_rustc_version(),
    super::env::KernelInformation::get_rust_toolchain()
  );
}

/// TODO
mod qemu_uart {
  use owo_colors::OwoColorize;

  /// TODO
  #[derive(Debug)]
  pub struct Logger {
    uart: crate::arch::drivers::uart::Uart,
  }

  impl Logger {
    /// ### Construct a New QEMU Logger
    ///
    /// This function creates a new instance of the QEMU
    /// logger structure.
    pub const fn new() -> Self {
      Self {
        uart: crate::arch::drivers::uart::Uart::new(0x1000_0000),
      }
    }
  }

  impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool { true }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
      use core::fmt::Write;

      if !self.enabled(record.metadata()) {
        return;
      }

      /// Shortens the log sequence (writing via `println!`).
      macro_rules! log_with_color {
        ($r:expr, $g:expr, $b:expr) => {{
          if let Err(_) = writeln!(
            crate::arch::drivers::uart::Uart::new(0x1_000_0000),
            "{} {}",
            record.level().as_str().fg_rgb::<$r, $g, $b>(),
            record.args()
          ) {
            // unsafe {
            //   super::LOGGER.enable_or_disable_qemu_uart(false);
            // }
          }
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
  }
}
