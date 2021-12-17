/// ## Hardware Abstractions
///
/// This module contains all hardware-specific code. Moreover,
/// architecture-specific code is also located here. This module is
/// initialized first after booting and starting the kernel.
mod hardware;

/// ## Generic Helper Function
///
/// This module provides generic function used by other modules, such
/// as
///
/// - logging
/// - not returning
/// - panicking
/// - testing
///
/// It also provides the test runners and the kernel version
/// information.
mod helper;

pub use helper::test_runner;
pub use helper::never_return;

/// ### Global Initialization
///
/// This function initializes the whole kernel. It takes care of
///
/// - printing important initial information
/// - calling the hardware initialization subroutine
pub fn init(boot_information: &bootloader::BootInfo)
{
	helper::display_initial_information(boot_information);
	crate::log_info!("Initialization started");

	hardware::init();

	crate::log_info!("Initialization finished");
}
