// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// * ARM 64 Bit
// * ---------------------------------------------------------------------

/// ## The ARM 64bit Architecture
///
/// This module contains ARM 64bit specific initialization and
/// setup code - compiled conditionally.
#[cfg(target_arch = "aarch64")]
mod _aarch64;

#[cfg(target_arch = "aarch64")] pub use _aarch64::cpu;

// * x86 32 Bit
// * ---------------------------------------------------------------------

/// ## The x86 32bit Architecture
///
/// This module contains x86 32bit specific initialization and
/// setup code - compiled conditionally.
#[cfg(target_arch = "i686")]
mod _i686;

#[cfg(target_arch = "i686")] pub use _i686::cpu;

// * x86 64 Bit
// * ---------------------------------------------------------------------

/// ## The `x86_64` Architecture
///
/// This module contains `x86_64` specific initialization and setup
/// code - compiled conditionally.
#[cfg(target_arch = "x86_64")]
mod _x86_64;

#[cfg(target_arch = "x86_64")] pub use _x86_64::cpu;
#[cfg(target_arch = "x86_64")] pub use _x86_64::memory;
