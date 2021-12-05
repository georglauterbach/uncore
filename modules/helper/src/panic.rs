#![allow(unused_imports)]

use core::panic::PanicInfo;
use crate::{
	never_return,
	qemu,
};

/// # Panic Handler when Running Tests
///
/// This function is marked for conditional compilation, and
/// is used when running the custom tests suite.
#[cfg(test)]
pub fn __panic(_panic_info: &PanicInfo) -> !
{
	// crate::serial_println!("[PANIC]\n{}\n", panic_info);
	qemu::exit(qemu::ExitCode::Failed);
	never_return()
}

/// # Panic Handler when not Running Tests
///
/// This function is marked for conditional compilation, and
/// is used when running the binary natively, i.e. not the
/// tests.
#[cfg(not(test))]
pub fn __panic(_panic_info: &PanicInfo) -> !
{
	// crate::serial_println!("[PANIC]\n{}\n", panic_info);
	never_return()
}
