/// ## Generic Bootloader Information
///
/// This module contains the (architecture independent) information
/// about the boot process and memory layout.
mod boot;

pub use boot::BootInformation;

/// ## Miscellaneous Helpers
///
/// Provides various of the most generic helper functions, such as
/// `never_return()`.
pub mod miscellaneous;

pub use miscellaneous::main;
pub use miscellaneous::never_return;

/// ## Provides the API for Panicking
///
/// This module provides the implementation for the panic macro and
/// panic behavior.
mod panic;

pub use panic::panic_callback;

/// ## Providing Support for Tests
///
/// This module provides the implementation to run tests. This
/// includes unit-tests as well as integration tests.
pub mod test;
