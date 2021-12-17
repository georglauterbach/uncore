/// ## Uniform Logging
///
/// This module exports the `log_!` macros with different log levels.
pub mod log;

/// ## Miscellaneous Helpers
///
/// Provides various of the most generic helper functions, such as
/// `never_return()`.
mod miscellaneous;

pub use miscellaneous::never_return;
pub(super) use miscellaneous::display_initial_information;

/// ## Provides the API for Panicking
///
/// This module provides the implementation for the panic macro and
/// panic behavior.
mod panic;

/// ## Providing Support for Tests
///
/// This module provides the implementation to run tests. This
/// includes unit-tests as well as integration tests.
mod test;
pub use test::test_runner;
