// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## The Global Kernel Logger
///
/// This static variable is used by the [`log`] crate for
/// logging kernel-wide.
pub static LOGGER: KernelLogger = KernelLogger {
	qemu_debug_logger:         qemu::Logger::new(),
	qemu_debug_logger_enabled: true,
	serial_logger:             serial::Logger::new(),
	serial_logger_enabled:     true,
};

/// ### The Main Kernel Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
pub struct KernelLogger
{
	/// We use the QEMU `debugcon` feature to log to a file
	/// located under `build/qemu/debugcon.txt`.
	qemu_debug_logger:         qemu::Logger,
	/// Indicates whether QEMU's `debugcon` feature should be
	/// enabled.
	qemu_debug_logger_enabled: bool,
	/// The serial interface is accessed via port-mapped I/O
	/// and forwarded by QEMU to standard output on the terminal.
	serial_logger:             serial::Logger,
	/// Indicates whether the serial interface should log
	/// messages.
	serial_logger_enabled:     bool,
}

impl KernelLogger
{
	/// ### Enable or Disable QEMU
	///
	/// This function enabled or disables the log for QEMU's
	/// `debugcon` feature.
	pub fn enable_or_disable_qemu(&mut self, state_as_bool: bool)
	{
		self.qemu_debug_logger_enabled = state_as_bool;
	}

	/// ### Enable or Disable QEMU
	///
	/// This function enabled or disables the log for the serial
	/// interface.
	pub fn enable_or_disable_serial(&mut self, state_as_bool: bool)
	{
		self.serial_logger_enabled = state_as_bool;
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

		if self.qemu_debug_logger_enabled {
			self.qemu_debug_logger.log(record);
		}
		if self.serial_logger_enabled {
			self.serial_logger.log(record);
		}
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

	crate::prelude::log_debug!("Enabled kernel logging");
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
		"Kernel was compiled at '{}'",
		KernelInformation::get_compilation_date_and_time()
	);
	log_trace!(
		"Kernel was compiled with rustc version '{}'",
		KernelInformation::get_rustc_version()
	);
	log_trace!(
		"Kernel was compiled with toolchain '{}'",
		KernelInformation::get_rust_toolchain()
	);
}

/// ## A Serial Device Interface
///
/// This module abstracts over the serial port (port-mapped I/O) and
/// provides a [`log`]-conform structure for the global logger to use.
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

	/// ### A Serial Port Interface
	///
	/// This structure abstracts over the serial port and logs
	/// messages on this port.
	pub struct Logger;

	impl Logger
	{
		/// ### Construct a New Serial Logger
		///
		/// This function creates a new instance of the serial
		/// logger structure.
		pub const fn new() -> Self { Self }

		/// ### Write to Serial Output
		///
		/// This function prints its arguments to the serial
		/// output.
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

	impl log::Log for Logger
	{
		fn enabled(&self, _: &log::Metadata) -> bool { true }

		fn flush(&self) {}

		fn log(&self, record: &log::Record)
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

			Self::write(format_args!(
				"[ {} ] {:>25.*}{}{:<4.*} | {}\n",
				log_level.fg(color),
				25,
				record.file().unwrap_or("unknown"),
				"@".fg(color),
				4,
				record.line().unwrap_or(0),
				record.args().fg(color)
			));
		}
	}
}

/// ## QEMU's `debugcon` Feature
///
/// This module abstracts over QEMU's `debugcon` feature and provides
/// a [`log`]-conform structure for the global logger to use.
mod qemu
{
	/// ### QEMU `debugcon` Logger
	///
	/// Implementation of a logger for the [`log`] crate, that
	/// writes everything to QEMU's "debugcon" feature, i.e. x86
	/// i/o-port 0xe9.
	pub struct Logger;

	impl Logger
	{
		/// ### Construct a New QEMU Logger
		///
		/// This function creates a new instance of the QEMU
		/// logger structure.
		pub const fn new() -> Self { Self }

		/// ### Write to the Correct Port
		///
		/// This function writes to the `0xE9` port
		/// (port-mapped I/O). It assumes that the output is
		/// valid ASCII. The data is not transformed to ASCII.
		pub fn write_to_debugcon_port(bytes: &str)
		{
			for byte in bytes.as_bytes() {
				unsafe { x86::io::outb(0xE9, *byte) };
			}
		}
	}

	impl log::Log for Logger
	{
		fn enabled(&self, _: &log::Metadata) -> bool { true }

		fn flush(&self) {}

		fn log(&self, record: &log::Record)
		{
			use ::core::fmt::Write;
			use log::Level;

			let mut buf = arrayvec::ArrayString::<16384>::new();

			// https://coolors.co/da3e52-f2e94e-a3d9ff-96e6b3-9fa4a8
			let log_level = match record.level() {
				Level::Error => " ERROR ",
				Level::Warn => "WARNING",
				Level::Info => "  INF  ",
				Level::Debug => " DEBUG ",
				Level::Trace => " TRACE ",
			};

			let result = write!(
				&mut buf,
				"[ {} ] {:>40.*}@{:<4.*} | {}\n",
				log_level,
				40,
				record.file().unwrap_or("unknown"),
				4,
				record.line().unwrap_or(0),
				record.args()
			);

			if let Err(error) = result {
				let mut buf = arrayvec::ArrayString::<256>::new();
				let _ = write!(buf, "QEMU debugcon error: {}", error);
				Self::write_to_debugcon_port("(fail-save log) | ");
				Self::write_to_debugcon_port(buf.as_str());
				Self::write_to_debugcon_port("\n");
			}

			Self::write_to_debugcon_port(buf.as_str());
		}
	}
}
