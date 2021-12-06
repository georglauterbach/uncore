use core::panic::PanicInfo;

/// ### Panic Handler when Running Tests
///
/// This function is marked for conditional compilation, and
/// is used when running the custom tests suite.
#[cfg(test)]
#[inline]
fn __panic(_panic_info: &PanicInfo) -> !
{
	crate::miscellaneous::qemu::exit_with_failure();
	crate::never_return()
}

/// ### Panic Handler when not Running Tests
///
/// This function is marked for conditional compilation, and
/// is used when running the binary natively, i.e. not the
/// tests.
#[cfg(not(test))]
#[inline]
fn __panic(_panic_info: &PanicInfo) -> ! { crate::never_return() }

/// ### Default Panic Handler
///
/// This function provides a very basic panic handler, that, depending
/// on whether you are running tests or not, writes an exit code and
/// does not return afterwards. Note that we do not unwind the stack.
#[panic_handler]
pub fn panic(panic_info: &core::panic::PanicInfo) -> ! { __panic(panic_info); }
