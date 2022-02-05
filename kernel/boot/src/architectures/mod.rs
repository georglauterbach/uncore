// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## The x86 64 Bit Architecture
///
/// This module contains x86 64 Bit specific initialization and setup
/// code - compiled conditionally.
#[cfg(target_arch = "x86_64")]
mod _x86_64;

#[cfg(target_arch = "x86_64")] pub use _x86_64::link_with_bootloader;
