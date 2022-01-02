/// ## Hardware Abstractions
///
/// This module contains all hardware-specific code. Moreover,
/// architecture-specific code is also located here. This module is
/// initialized first after booting and starting the kernel.
pub mod hardware;

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

pub use helper::miscellaneous;
pub use helper::test;

pub use helper::main;
pub use helper::panic_callback;

pub use helper::BootInformation;

/// ## Uniform Logging
///
/// This module exports the `log_!` macros with different log levels.
pub mod log;

/// ## Virtual Memory Implementation
///
/// Generic virtual memory implementation that bases upon the
/// architecture-specific implementation.
mod memory;
