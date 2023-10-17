// SPDX-License-Identifier: GPL-3.0-or-later

/// ## Create a Bootable Image
///
/// This module contains the [`bootloader::link_with_bootloader`] function to link a
/// kernel binary or a test binary with the bootloader to be able to tun in QEMU.
pub mod bootloader;

/// ## Build Primitives
///
/// Contains general functions when building `unCORE`, like [`build::set_target`] to set
/// the LLVM build target.
pub mod build;

/// ## Environment Variables
///
/// Contains function to cope with the tedious environment setup. This module unifies the
/// setup and provides consistency across all crates.
pub mod environment;

/// ## Provides Logging Functionality
///
/// A shameless copy of the kernel logging implementation.
pub mod logger;
