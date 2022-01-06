// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Boot Code
///
/// This module holds the initialization boot code for different
/// architectures. These are written in assembly and includes in the
/// `mod.rs` file via conditional compilation and the `global_asm!`
/// macro.
mod boot;

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

/// ## The Kernel Prelude
///
/// This module provides
///
/// - modules
/// - structures
/// - macros
/// - function
///
/// used by many other modules. It should be imported via
///
/// ``` rust
/// use library::prelude::*;
/// ```
pub mod prelude;

/// ## Handle UEFI
///
/// Handles UEFI related matters. After entry into
/// `crate::kernel_main(...)`, UEFI boot services are still active and
/// we need to handle and exit them.
///
/// ### Trivia
///
/// The module name was chosen in order to not conflict with the
/// `uefi` crate.
mod __uefi;
