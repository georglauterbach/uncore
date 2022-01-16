// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// * ARM 64 Bit
// * ---------------------------------------------------------------------

/// ## The ARM 64 Bit Architecture
///
/// This module contains ARM 64bit specific initialization and
/// setup code - compiled conditionally.
#[cfg(target_arch = "aarch64")]
mod _aarch64;

#[cfg(target_arch = "aarch64")] pub use _aarch64::initialize;

// * x86 32 Bit
// * ---------------------------------------------------------------------

/// ## The x86 32 Bit Architecture
///
/// This module contains x86 32bit specific initialization and
/// setup code - compiled conditionally.
#[cfg(target_arch = "i686")]
mod _i686;

#[cfg(target_arch = "i686")] pub use _i686::initialize;

// * x86 64 Bit
// * ---------------------------------------------------------------------

/// ## The x86 64 Bit Architecture
///
/// This module contains x86 64 Bit specific initialization and setup
/// code - compiled conditionally.
#[cfg(target_arch = "x86_64")]
mod _x86_64;

#[cfg(target_arch = "x86_64")] pub use _x86_64::initialize;
