/// ## CPU Exception Callbacks
///
/// This module contains the callback functions executed when a CPU
/// exception is thrown.
mod exceptions;

/// ## General CPU Initialization
///
/// This module contains code for general setup code, such as Global
/// Descriptor Table (GDT) setup.
mod general;

/// ## CPU Interrupt Callbacks
///
/// This module contains the callback functions executed when CPU
/// interrupts arrive.
mod interrupts;

/// ## Virtual Memory Implementation
///
/// This module contains the virtual memory / paging abstractions for
/// `x86_64`.
pub mod memory;

/// ### `x86_64` Setup
///
/// The `x86_64`-specific setup is handled by this function.
pub fn init()
{
	crate::log_trace!("Architecture specific initialization started");
	crate::log_trace!("Running on x68_64");

	general::gdt::init();
	interrupts::init();
}
