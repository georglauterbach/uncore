// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use ::core::{
	cell,
	fmt,
};
use crate::prelude::*;

lazy_static::lazy_static! {

	/// ### The Logging Severity Level
	///
	/// The `LOG_LEVEL` is used to define which messages are being logged.
	/// All messaged with an equal or higher priority are logged when not
	/// running tests. When running tests, all messages with a severity of
	/// `Level::Warning` or higher are logged.
	static ref LOG_LEVEL: spin::Mutex<cell::RefCell<log::Level>> =
		spin::Mutex::new(cell::RefCell::new(log::Level::Trace));
}

static LOGGER: KernelLog = KernelLog {};

/// ### The Main Kernel Logger
///
/// This structure holds associated function that provide logging. The
/// `log::Log` trait is implemented for this structure.
pub struct KernelLog;

impl KernelLog
{
	/// ### Pretty Logs
	///
	/// This function takes care of formatting and formatting
	/// only. It introduces colors abd sets the log format.
	fn format_arguments(level: log::Level, target: &str, message: &fmt::Arguments)
	{
		use ansi_rgb::Foreground;
		use log::Level;
		use rgb::RGB8;

		// https://coolors.co/da3e52-f2e94e-a3d9ff-96e6b3-4e5356
		let red = RGB8::new(218, 62, 82);
		let yellow = RGB8::new(242, 233, 78);
		let blue = RGB8::new(163, 217, 255);
		let green = RGB8::new(150, 230, 179);
		let grey = RGB8::new(78, 83, 86);

		let level = match level {
			Level::Error => level.fg(red),
			Level::Warn => level.fg(yellow),
			Level::Info => level.fg(blue),
			Level::Debug => level.fg(green),
			Level::Trace => level.fg(grey),
		};

		Self::write(&format_args!("{} - {}  | {}", level, target, message));
	}

	/// ### Show Initial Information
	///
	/// This function sets the log level and displays version and
	/// bootloader information.
	pub fn init(log_level: Option<log::Level>)
	{
		if let Some(level) = log_level {
			Self::set_log_level(level);
		};

		log::set_logger(&LOGGER).unwrap();

		display_initial_information();
		log_info!("Post-UEFI initialization started");
	}

	/// ### Set the Kernel Log Level
	///
	/// This function adjusts the kernel log level. Only call this
	/// function once and at the very start if necessary.
	fn set_log_level(new_log_level: log::Level)
	{
		*LOG_LEVEL.lock().borrow_mut() = new_log_level;
	}

	/// ### Write the Log to All Outputs
	///
	/// This function just forwards the arguments to all loggers.
	fn write(arguments: &fmt::Arguments) { serial::write(arguments); }
}

impl log::Log for KernelLog
{
	fn enabled(&self, metadata: &log::Metadata) -> bool
	{
		metadata.level() <= *LOG_LEVEL.lock().borrow()
	}

	fn log(&self, record: &log::Record)
	{
		if !self.enabled(record.metadata()) {
			return;
		}

		Self::format_arguments(record.level(), record.target(), record.args());
	}

	fn flush(&self) {}
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
	log_info!("This is unCORE {}\n", KernelInformation::get_version());

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

	/// ### I/O Port
	///
	/// On the x86 architecture, there is a UART controller chip
	/// behind this I/O port.
	const SERIAL_IO_PORT: u16 = 0x3F8;

	/// ### Serial Writer
	///
	/// With this port, we can write to the serial output.
	static SERIAL0: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
		let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
		serial_port.init();
		Mutex::new(serial_port)
	});

	/// ### Write to Serial Output
	///
	/// This function prints its arguments to the serial output.
	pub(super) fn write(arguments: &::core::fmt::Arguments)
	{
		use core::fmt::Write;
		use x86_64::instructions::interrupts;

		interrupts::without_interrupts(|| {
			SERIAL0.lock()
				.write_fmt(*arguments)
				.expect("Printing to serial failed");
		});
	}
}
