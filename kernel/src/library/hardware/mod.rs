/// ## Architectural Differences
///
/// This module contains architecture specific initialization code and
/// uses conditional compilation.
mod architectures;

/// ### Hardware Initialization
///
/// This function initializes the hardware module by
///
/// 1. Calling the architecture-specific initialization subroutines
pub(super) fn init()
{
	crate::log_debug!("Initializing hardware");

	architectures::init();
}
