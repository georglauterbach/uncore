/// ## Architectural Differences
///
/// This module contains architecture specific initialization code and
/// uses conditional compilation.
mod architectures;

/// ## Virtual Memory Implementation
///
/// Generic virtual memory implementation that bases upon the
/// architecture-specific implementation.
mod memory;

/// ### Hardware Initialization
///
/// This function initializes the hardware module by
///
/// 1. Calling the architecture-specific initialization subroutines
pub(super) fn init(boot_information: &bootloader::BootInfo)
{
	crate::log_debug!("Initializing hardware");

	architectures::init();
	memory::init(boot_information);
}
