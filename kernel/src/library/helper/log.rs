use ::core::fmt;

/// ### The Logging Severity Level
///
/// The `LOG_LEVEL` is used to define which messages are being logged.
/// All messaged with an equal or higher priority are logged when not
/// running tests. When running tests, all messages with a severity of
/// `Level::Warning` or higher are logged.
const LOG_LEVEL: Level = if super::test::IS_TEST {
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

/// ### Log Indirection
///
/// An indirection that is used in order to make it easy to switch to
/// different loggers or log to multiple interfaces. The indirection
/// is nice because now we do not need to make the different output
/// modules (such as `serial`) in this module public.
#[doc(hidden)]
pub fn __log(log_level: Level, arguments: fmt::Arguments)
{
	if log_level < LOG_LEVEL {
		return;
	}

	serial::print(format_args!("{}{}\n", log_level, arguments));
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::None,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Trace,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Debug,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Info,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Warning,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Error,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Fatal,
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
		$crate::library::helper::log::__log(
			$crate::library::helper::log::Level::Test,
			format_args!($($arg)*)
		)
	};
}

/// ## A Serial Device Interface
///
/// The `write` module makes heavy use of this module
/// as it `serial` provides the ability to write to
/// a serial device which is "forwarded" to `stdout`
/// via QEMU.
mod serial
{
	use spin::{
		Lazy,
		Mutex,
	};
	use uart_16550::SerialPort;

	/// ### Serial Writer
	///
	/// With this port, we can write to the serial output.
	static SERIAL0: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
		let mut serial_port = unsafe { SerialPort::new(0x3F8) };
		serial_port.init();
		Mutex::new(serial_port)
	});

	/// ### Write to Serial Output
	///
	/// This function prints its arguments to the serial output.
	pub(super) fn print(arguments: ::core::fmt::Arguments)
	{
		use core::fmt::Write;
		use x86_64::instructions::interrupts;

		interrupts::without_interrupts(|| {
			SERIAL0.lock()
				.write_fmt(arguments)
				.expect("Printing to serial failed");
		});
	}
}
