// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Architecture Specific Code
///
/// Holds **all** _architecture dependent_ code. That includes (but
/// not exclusively)
///
/// - assembly boot code
/// - CPU initialization code
/// - virtual memory initialization code
pub mod architectures;

/// ## Boot Code
///
/// This module holds boot code concerning multiboot2 and UEFI. The
/// multiboot2 information structure is parsed and the UEFI boot
/// services are exited.
pub mod boot;

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
