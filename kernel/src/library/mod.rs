// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Boot Code
///
/// This module holds the initialization boot code for the `x86_32`
/// architecture. The "real" boot code is written in assembly. The
/// multiboot2 and UEFI wrappers also reside here.
pub mod boot;

/// ## Hardware Specific Code
///
/// First and foremost, holds the CPU initialization routines for
/// post-boot startup.
mod hardware;

/// ## Generic Helper Function
///
/// Holds many of the generic functions re-exported in the `prelude`
/// module. These include panic callbacks, test infrastructure or
/// information about the kernel.
mod helper;

/// ## Uniform Logging
///
/// This module exports the `log_!` macros with different log levels.
/// It implements the `log` crate's logging facade.
pub mod log;

/// ## Virtual Memory
///
/// This module handles virtual memory, that is (demand) paging,
/// allocations, etc. for the user- and kernel-space.
mod memory;

/// ## The Kernel Prelude
///
/// This module provides common
///
/// - modules
/// - structures
/// - macros
/// - functions
///
/// used by many other modules. It should be imported via
///
/// ``` edition2021
/// use library::prelude::*;
/// ```
pub mod prelude;
