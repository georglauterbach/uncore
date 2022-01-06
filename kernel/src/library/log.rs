// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use ::core::{
	cell,
	fmt,
};
use crate::prelude::*;

static LOGGER: qemu::QemuDebugLogger = qemu::QemuDebugLogger::new();

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
	fn format_arguments(record: &log::Record)
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

		let level = record.level();
		let level = match level {
			Level::Error => level.fg(red),
			Level::Warn => level.fg(yellow),
			Level::Info => level.fg(blue),
			Level::Debug => level.fg(green),
			Level::Trace => level.fg(grey),
		};

		Self::write(&format_args!(
			"{:>7} {:>15}@{:<4} | {}",
			level,
			record.file().unwrap_or("unknown"),
			record.line().unwrap_or(0),
			record.args()
		));
	}

	/// ### Show Initial Information
	///
	/// This function sets the log level and displays version and
	/// bootloader information.
	pub fn init(log_level: Option<log::Level>)
	{
		if let Some(level) = log_level {
			log::set_max_level(level.to_level_filter())
		};

		log::set_logger(&LOGGER).unwrap();

		display_initial_information();
		log_info!("Post-UEFI initialization started");
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
		// metadata.level() <= *LOG_LEVEL.lock().borrow()
		true
	}

	fn log(&self, record: &log::Record)
	{
		if !self.enabled(record.metadata()) {
			return;
		}

		Self::format_arguments(record);
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

mod qemu
{

	use core::fmt::Write;
	use log::{Metadata, Record};
	use uefi::{CStr16, Char16};

	/// Implementation of a logger for the [`log`] crate, that
	/// writes everything to QEMUs "debugcon" feature, i.e. x86
	/// i/o-port 0xe9.
	pub struct QemuDebugLogger {}

	impl QemuDebugLogger
	{
		pub const fn new() -> Self { Self {} }
	}

	impl log::Log for QemuDebugLogger
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
				let _ = write!(
					buf,
					"QemuDebugLoggerError({})",
					e
				);
				qemu_debug_stdout_str("QemuDebugLogger: ");
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
