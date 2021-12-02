use crate::core_lib::misc::helper::qemu;
use core::panic::PanicInfo;
use crate::core_lib::misc::helper;

/// # Panic Handler when not Running Tests
///
/// This function is marked for conditional compilation, and is used
/// when running the binary natively, i.e. not the tests.
#[cfg(not(test))]
pub fn panic(info: &PanicInfo) -> !
{
	crate::println!("\n{{{{ PANIC }}}}\n\n{}\n\n", info);

	helper::__never_return()
}

/// # Panic Handler when Running Tests
///
/// This function is marked for conditional compilation, and is used
/// when running the custom tests suite.
#[cfg(test)]
pub fn panic(info: &PanicInfo) -> ! { test_panic_handler(info) }

/// # Publicly Available Panic Handler
///
/// This function makes sure we have a publicly
/// available panic handler function which we could
/// call from integration tests.
pub fn test_panic_handler(info: &PanicInfo) -> !
{
	crate::serial_println!("[Failed]\n\n{{{{ PANIC }}}}\n\n{}\n\n", info);
	qemu::_exit(qemu::ExitCode::Failed);

	helper::__never_return()
}
