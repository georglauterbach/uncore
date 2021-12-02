/// # `SemVer` Version
///
/// This is the semantic versioning version string with added release
/// candidate, maturity grade and stability identifier.
const VERSION: &str = "v0.1.0-rc1 alpha1 unstable";

/// # The Hardware Interface
///
/// The `hw` module provides support for and abstraction over hardware
/// access. This mainly includes interaction with the Central Processing Unit
/// (CPU) and I/O devices.
pub mod hw;

/// # Memory Management
///
/// This module handles all memory management. This includes paging and
/// allocation of all sorts, for example on the heap.
pub mod mem;

/// # Other Functionality
///
/// All functions and methods that are rather generic and serve general
/// purposes are placed in `misc`. The high-level implementation for
/// panicking is found here as well as various helper functions.
pub mod misc;

/// # Providing Support for Tests
///
/// This module provides the implementation to run tests. This includes
/// unit-tests as well as integration tests.
pub mod tests;
