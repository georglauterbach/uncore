// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// ? HARDWARE PLATFORMS
// ? ---------------------------------------------------------------------

// * RISC V
// * ---------------------------------------------------------------------

/// ## The RISC V 64bit Architecture
///
/// This module contains RISC V 64bit specific initialization and
/// setup code. Compiled conditionally.
#[cfg(target_arch = "riscv64")]
mod _riscv64;

#[cfg(target_arch = "riscv64")] pub use _riscv64::cpu;

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
