// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// Each hardware platform has its own, conditionally compiled module
// that contains an `initialization()` function used in
// `kernel_main()` to set up the kernel.

// ? HARDWARE PLATFORMS
// ? ---------------------------------------------------------------------

// * x86_32
// * ---------------------------------------------------------------------

/// ## The `x86_32` Architecture
///
/// This module contains `x86_32` specific initialization and setup
/// code. Compiled conditionally.
#[cfg(target_arch = "x86_32")]
mod _x86_32;

#[cfg(target_arch = "x86_32")] pub use _x86_32::cpu;

// * x86_64
// * ---------------------------------------------------------------------

/// ## The `x86_64` Architecture
///
/// This module contains `x86_64` specific initialization and setup
/// code. Compiled conditionally.
#[cfg(target_arch = "x86_64")]
mod _x86_64;

#[cfg(target_arch = "x86_64")] pub use _x86_64::cpu;
#[cfg(target_arch = "x86_64")] pub use _x86_64::memory;
