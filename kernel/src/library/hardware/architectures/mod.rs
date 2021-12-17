/// ## The `x86_64` Architecture
///
/// This module contains `x86_64` specific initialization and setup
/// code. Compiled conditionally.
#[cfg(target_arch = "x86_64")]
mod arch_x86_64;

#[cfg(target_arch = "x86_64")] pub use arch_x86_64::init;
#[cfg(target_arch = "x86_64")] pub use arch_x86_64::memory;

/// ## The `x86_32` Architecture
///
/// This module contains `x86_32` specific initialization and setup
/// code. Compiled conditionally.
#[cfg(target_arch = "x86_32")]
mod arch_x86_32;

#[cfg(target_arch = "x86_32")] pub use arch_x86_32::init;
#[cfg(target_arch = "x86_32")] pub use arch_x86_64::memory;
