// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## The Global Kernel Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide.
pub static LOGGER: KernelLogger = KernelLogger {
	qemu_debug_logger: qemu::Logger::new(),
};

/// ### The Main Kernel Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
pub struct KernelLogger
{
	/// We use the QEMU `debugcon` feature to log to a file
	/// located under `build/qemu/debugcon.txt`.
	qemu_debug_logger: qemu::Logger,
}

impl KernelLogger
{
	/// ### Pretty Logs
	///
	/// This function takes care of formatting the log record
	/// and dispatching it all available loggers. It introduces
	/// colors and sets the log format for certain logs.
	fn format_and_write_arguments(record: &log::Record)
	{
		use ansi_rgb::Foreground;
		use log::Level;
		use rgb::RGB8;

		// https://coolors.co/da3e52-f2e94e-a3d9ff-96e6b3-9fa4a8
		let (log_level, color) = match record.level() {
			Level::Error => (" ERROR ", RGB8::new(218, 62, 82)),
			Level::Warn => ("WARNING", RGB8::new(242, 233, 78)),
			Level::Info => ("  INF  ", RGB8::new(163, 217, 255)),
			Level::Debug => (" DEBUG ", RGB8::new(150, 230, 179)),
			Level::Trace => (" TRACE ", RGB8::new(159, 164, 168)),
		};

		serial::write(format_args!(
			"[ {} ] {:>20.*}{}{:<4.*} | {}\n",
			log_level.fg(color),
			20,
			record.file().unwrap_or("unknown"),
			"@".fg(color),
			4,
			record.line().unwrap_or(0),
			record.args().fg(color)
		));
	}
}

impl log::Log for KernelLogger
{
	fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

	fn log(&self, record: &log::Record)
	{
		if !self.enabled(record.metadata()) {
			return;
		}

		Self::format_and_write_arguments(record);
		self.qemu_debug_logger.log(record);
	}

	fn flush(&self) {}
}

/// ### Show Initial Information
///
/// This function sets the log level and displays version and
/// bootloader information.
pub fn init(log_level: Option<log::Level>)
{
	if let Some(level) = log_level {
		log::set_max_level(level.to_level_filter());
	}

	log::set_logger(&LOGGER).expect("Log should not have already been set");
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
	use crate::prelude::*;

	log_info!("This is unCORE {}", KernelInformation::get_version());

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
	pub(super) fn write(arguments: ::core::fmt::Arguments)
	{
		use ::core::fmt::Write;

		x86_64::instructions::interrupts::without_interrupts(|| {
			SERIAL0.lock()
				.write_fmt(arguments)
				.expect("Printing to serial failed");
		});
	}
}

/// ## QEMU's `debugcon` Feature
///
/// This module abstracts over QEMU's `debugcon` feature and provides
/// a [`log`]-conform structure for the global logger to use.
mod qemu
{
	// TODO needs formatting
	use core::fmt::Write;
	use log::{
		Metadata,
		Record,
	};
	use uefi::{
		CStr16,
		Char16,
	};

	/// Implementation of a logger for the [`log`] crate, that
	/// writes everything to QEMUs "debugcon" feature, i.e. x86
	/// i/o-port 0xe9.
	pub struct Logger;

	impl Logger
	{
		pub const fn new() -> Self { Self }
	}

	impl log::Log for Logger
	{
		fn enabled(&self, _metadata: &Metadata) -> bool { true }

		fn log(&self, record: &Record)
		{
			let mut buf = arrayvec::ArrayString::<16384>::new();

			let res = writeln!(
				&mut buf,
				"[{:>5}] {:>15}@{}: {}",
				record.level(),
				record.file().unwrap_or("<unknown file>"),
				record.line().unwrap_or(0),
				record.args()
			);
			if let Err(e) = res {
				let mut buf = arrayvec::ArrayString::<256>::new();
				let _ = write!(buf, "QemuDebugLoggerError({})", e);
				qemu_debug_stdout_str("Logger: ");
				qemu_debug_stdout_str(buf.as_str());
				qemu_debug_stdout_str("\n");
				// panic_error!(BootError::
				// PanicStackArrayTooSmall, "");
			}

			// in any way, write the string as far as it was formatted (even if it
			// failed in the middle)
			qemu_debug_stdout_str(buf.as_str());
		}

		fn flush(&self) {}
	}

	pub fn qemu_debug_stdout_str(msg: &str) { qemu_debug_stdout_u8_arr(msg.as_bytes()); }

	#[allow(unused)]
	pub fn qemu_debug_stdout_c16str(msg: &CStr16)
	{
		msg.iter().for_each(|c: &Char16| {
			let val: u16 = (*c).into();
			qemu_debug_stdout_u8_arr(&val.to_be_bytes());
		});
	}

	/// Assumes that the output is valid ASCII.
	/// Data is not transformed to ASCII.
	pub fn qemu_debug_stdout_u8_arr(bytes: &[u8])
	{
		for byte in bytes {
			unsafe { x86::io::outb(0xE9, *byte) };
		}
	}
	#[allow(unused)]
	pub fn qemu_debug_stdout_char_arr(chars: &[char])
	{
		for char in chars {
			unsafe { x86::io::outb(0xE9, *char as u8) };
		}
	}
}
